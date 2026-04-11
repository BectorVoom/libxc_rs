//! Mixed functional accumulation logic.
//!
//! Mixed/hybrid functionals evaluate multiple auxiliary functionals and combine
//! their weighted results. This module provides the low-level accumulation
//! primitive (`add_to_mix`) and the mixed LDA evaluation loop
//! (`evaluate_mixed_lda`), matching libxc's `mix_func.c` behavior.

use crate::dims::Dimensions;
use crate::error::LibxcRsError;
use crate::eval::dispatch::dispatch_lda;
use crate::eval::workspace::EvaluationWorkspace;
use crate::input::LdaInput;
use crate::model::{DerivativeOrder, Thresholds};
use crate::output::LdaOutput;

/// Configuration for one auxiliary functional in a mixed evaluation.
pub struct AuxiliaryConfig {
    /// The functional's alpha parameter (e.g., 1.0 for LDA_X).
    pub alpha: f64,
    /// The mixing coefficient (weight) for this auxiliary.
    pub weight: f64,
    /// Numerical thresholds for this auxiliary's evaluation.
    pub thresholds: Thresholds,
}

/// Weighted accumulation: `dst[i] += coeff * src[i]` for all elements.
///
/// Matches libxc `mix_func.c` line 54 exactly.
/// The compiler will auto-vectorize this loop.
pub fn add_to_mix(dst: &mut [f64], coeff: f64, src: &[f64]) {
    debug_assert_eq!(dst.len(), src.len(), "add_to_mix: dst and src must have equal length");
    for (d, s) in dst.iter_mut().zip(src.iter()) {
        *d += coeff * *s;
    }
}

/// Evaluate a mixed LDA functional by dispatching to auxiliary functionals
/// and accumulating weighted results.
///
/// For each auxiliary:
/// 1. Zeros workspace scratch (prevents cross-contamination, T-03-07)
/// 2. Evaluates the auxiliary into scratch via `dispatch_lda`
/// 3. Accumulates `weight * scratch` into the caller's output
///
/// # Arguments
/// * `input` - LDA input bundle
/// * `order` - Maximum derivative order to compute
/// * `output` - Output bundle where weighted results are accumulated
/// * `auxiliaries` - Slice of auxiliary functional configurations
/// * `workspace` - Pre-allocated scratch workspace
///
/// # Errors
/// Returns error if workspace dimensions don't match input, or if any
/// auxiliary dispatch fails.
pub fn evaluate_mixed_lda(
    input: &LdaInput,
    order: DerivativeOrder,
    output: &mut LdaOutput,
    auxiliaries: &[AuxiliaryConfig],
    workspace: &mut EvaluationWorkspace,
) -> Result<(), LibxcRsError> {
    // Validate workspace matches input
    if workspace.np() != input.np() || workspace.spin() != input.spin() {
        return Err(LibxcRsError::WorkspaceMismatch {
            expected_np: input.np(),
            actual_np: workspace.np(),
            expected_spin: input.spin(),
            actual_spin: workspace.spin(),
        });
    }

    let np = input.np();
    let dims = Dimensions::lda(input.spin());

    // Zero the caller's output buffers before accumulation
    if let Some(ref mut buf) = output.zk {
        buf.fill(0.0);
    }
    if let Some(ref mut buf) = output.vrho {
        buf.fill(0.0);
    }
    if let Some(ref mut buf) = output.v2rho2 {
        buf.fill(0.0);
    }
    if let Some(ref mut buf) = output.v3rho3 {
        buf.fill(0.0);
    }
    if let Some(ref mut buf) = output.v4rho4 {
        buf.fill(0.0);
    }

    let zk_len = dims.zk as usize * np;
    let vrho_len = dims.vrho as usize * np;
    let v2rho2_len = dims.v2rho2 as usize * np;
    let v3rho3_len = dims.v3rho3 as usize * np;
    let v4rho4_len = dims.v4rho4 as usize * np;

    for aux in auxiliaries {
        // Zero scratch before each auxiliary (T-03-07 mitigation).
        // Note: dispatch_lda also zeros its output buffers on entry.
        // The double-zero is intentional -- this call prevents
        // cross-contamination between the accumulation readback and the
        // next dispatch_lda call, while dispatch_lda's internal zero
        // keeps it self-contained for direct (non-mixed) callers.
        workspace.zero_scratch();

        // Evaluate auxiliary into workspace scratch.
        // We need to build an LdaOutput pointing into the scratch,
        // call dispatch_lda, then drop the LdaOutput so we can
        // re-borrow scratch for accumulation.
        {
            let scratch = workspace.lda_scratch_mut();
            let mut scratch_output = LdaOutput {
                zk: Some(scratch.zk),
                vrho: if order >= DerivativeOrder::Vxc {
                    Some(scratch.vrho)
                } else {
                    None
                },
                v2rho2: if order >= DerivativeOrder::Fxc {
                    Some(scratch.v2rho2)
                } else {
                    None
                },
                v3rho3: if order >= DerivativeOrder::Kxc {
                    Some(scratch.v3rho3)
                } else {
                    None
                },
                v4rho4: if order >= DerivativeOrder::Lxc {
                    Some(scratch.v4rho4)
                } else {
                    None
                },
            };

            dispatch_lda(input, order, &mut scratch_output, aux.alpha, &aux.thresholds)?;
        }
        // scratch_output is dropped here, releasing the mutable borrow on workspace

        // Accumulate weighted scratch results into caller output.
        // Re-borrow scratch immutably via lda_scratch_mut (we need the slices again).
        let scratch = workspace.lda_scratch_mut();

        if let Some(ref mut dst) = output.zk {
            add_to_mix(dst, aux.weight, &scratch.zk[..zk_len]);
        }
        if order >= DerivativeOrder::Vxc
            && let Some(ref mut dst) = output.vrho
        {
            add_to_mix(dst, aux.weight, &scratch.vrho[..vrho_len]);
        }
        if order >= DerivativeOrder::Fxc
            && let Some(ref mut dst) = output.v2rho2
        {
            add_to_mix(dst, aux.weight, &scratch.v2rho2[..v2rho2_len]);
        }
        if order >= DerivativeOrder::Kxc
            && let Some(ref mut dst) = output.v3rho3
        {
            add_to_mix(dst, aux.weight, &scratch.v3rho3[..v3rho3_len]);
        }
        if order >= DerivativeOrder::Lxc
            && let Some(ref mut dst) = output.v4rho4
        {
            add_to_mix(dst, aux.weight, &scratch.v4rho4[..v4rho4_len]);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eval::dispatch::dispatch_lda;
    use crate::input::LdaInput;
    use crate::model::{DerivativeOrder, Spin, Thresholds};
    use crate::output::LdaOutput;

    fn default_thresholds() -> Thresholds {
        Thresholds::default()
    }

    // ── add_to_mix unit tests ─────────────────────────────────────

    #[test]
    fn add_to_mix_basic() {
        let mut dst = vec![0.0, 0.0];
        let src = vec![1.0, 2.0];
        add_to_mix(&mut dst, 0.7, &src);
        assert!((dst[0] - 0.7).abs() < 1e-15);
        assert!((dst[1] - 1.4).abs() < 1e-15);
    }

    #[test]
    fn add_to_mix_complementary_weights_sum_to_identity() {
        let mut dst = vec![0.0, 0.0];
        let src = vec![1.0, 2.0];
        add_to_mix(&mut dst, 0.7, &src);
        add_to_mix(&mut dst, 0.3, &src);
        assert!((dst[0] - 1.0).abs() < 1e-15, "dst[0] = {}", dst[0]);
        assert!((dst[1] - 2.0).abs() < 1e-15, "dst[1] = {}", dst[1]);
    }

    #[test]
    fn add_to_mix_accumulates_on_existing() {
        let mut dst = vec![10.0, 20.0];
        let src = vec![1.0, 2.0];
        add_to_mix(&mut dst, 0.5, &src);
        assert!((dst[0] - 10.5).abs() < 1e-15);
        assert!((dst[1] - 21.0).abs() < 1e-15);
    }

    // ── evaluate_mixed_lda integration tests ──────────────────────

    #[test]
    fn mixed_single_aux_weight_1_matches_dispatch() {
        let rho = vec![0.1, 0.2, 0.5, 1.0];
        let np = 4;
        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();

        // Direct dispatch
        let mut zk_direct = vec![0.0f64; np];
        let mut vrho_direct = vec![0.0f64; np];
        {
            let mut out_direct = LdaOutput::new(
                Some(&mut zk_direct), Some(&mut vrho_direct), None, None, None,
                np, Spin::Unpolarized,
            ).unwrap();
            dispatch_lda(&input, DerivativeOrder::Vxc, &mut out_direct, 1.0, &default_thresholds()).unwrap();
        }

        // Mixed with single aux, weight=1.0
        let mut zk_mixed = vec![0.0f64; np];
        let mut vrho_mixed = vec![0.0f64; np];
        let mut ws = EvaluationWorkspace::new(np, Spin::Unpolarized);
        {
            let mut out_mixed = LdaOutput::new(
                Some(&mut zk_mixed), Some(&mut vrho_mixed), None, None, None,
                np, Spin::Unpolarized,
            ).unwrap();
            let auxes = vec![AuxiliaryConfig {
                alpha: 1.0,
                weight: 1.0,
                thresholds: default_thresholds(),
            }];
            evaluate_mixed_lda(&input, DerivativeOrder::Vxc, &mut out_mixed, &auxes, &mut ws).unwrap();
        }

        for i in 0..np {
            assert!(
                (zk_mixed[i] - zk_direct[i]).abs() < 1e-15,
                "zk[{i}]: mixed={} vs direct={}", zk_mixed[i], zk_direct[i]
            );
            assert!(
                (vrho_mixed[i] - vrho_direct[i]).abs() < 1e-15,
                "vrho[{i}]: mixed={} vs direct={}", vrho_mixed[i], vrho_direct[i]
            );
        }
    }

    #[test]
    fn mixed_two_auxes_complementary_weights_match_dispatch() {
        let rho = vec![0.1, 0.5, 1.0];
        let np = 3;
        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();

        // Direct dispatch (weight 1.0)
        let mut zk_direct = vec![0.0f64; np];
        {
            let mut out_direct = LdaOutput::new(
                Some(&mut zk_direct), None, None, None, None,
                np, Spin::Unpolarized,
            ).unwrap();
            dispatch_lda(&input, DerivativeOrder::Exc, &mut out_direct, 1.0, &default_thresholds()).unwrap();
        }

        // Mixed with two auxes: 0.7 + 0.3 = 1.0
        let mut zk_mixed = vec![0.0f64; np];
        let mut ws = EvaluationWorkspace::new(np, Spin::Unpolarized);
        {
            let mut out_mixed = LdaOutput::new(
                Some(&mut zk_mixed), None, None, None, None,
                np, Spin::Unpolarized,
            ).unwrap();
            let auxes = vec![
                AuxiliaryConfig { alpha: 1.0, weight: 0.7, thresholds: default_thresholds() },
                AuxiliaryConfig { alpha: 1.0, weight: 0.3, thresholds: default_thresholds() },
            ];
            evaluate_mixed_lda(&input, DerivativeOrder::Exc, &mut out_mixed, &auxes, &mut ws).unwrap();
        }

        for i in 0..np {
            assert!(
                (zk_mixed[i] - zk_direct[i]).abs() < 1e-14,
                "zk[{i}]: mixed={} vs direct={}", zk_mixed[i], zk_direct[i]
            );
        }
    }

    #[test]
    fn mixed_half_weight_produces_half_result() {
        let rho = vec![0.5, 1.0];
        let np = 2;
        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();

        // Direct dispatch
        let mut zk_direct = vec![0.0f64; np];
        {
            let mut out_direct = LdaOutput::new(
                Some(&mut zk_direct), None, None, None, None,
                np, Spin::Unpolarized,
            ).unwrap();
            dispatch_lda(&input, DerivativeOrder::Exc, &mut out_direct, 1.0, &default_thresholds()).unwrap();
        }

        // Mixed with weight=0.5
        let mut zk_mixed = vec![0.0f64; np];
        let mut ws = EvaluationWorkspace::new(np, Spin::Unpolarized);
        {
            let mut out_mixed = LdaOutput::new(
                Some(&mut zk_mixed), None, None, None, None,
                np, Spin::Unpolarized,
            ).unwrap();
            let auxes = vec![AuxiliaryConfig {
                alpha: 1.0,
                weight: 0.5,
                thresholds: default_thresholds(),
            }];
            evaluate_mixed_lda(&input, DerivativeOrder::Exc, &mut out_mixed, &auxes, &mut ws).unwrap();
        }

        for i in 0..np {
            let expected = zk_direct[i] * 0.5;
            assert!(
                (zk_mixed[i] - expected).abs() < 1e-15,
                "zk[{i}]: mixed={} vs expected={}", zk_mixed[i], expected
            );
        }
    }

    #[test]
    fn mixed_vxc_order_populates_both_zk_and_vrho() {
        let rho = vec![0.5];
        let np = 1;
        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();

        let mut zk = vec![0.0f64; np];
        let mut vrho = vec![0.0f64; np];
        let mut ws = EvaluationWorkspace::new(np, Spin::Unpolarized);
        {
            let mut out = LdaOutput::new(
                Some(&mut zk), Some(&mut vrho), None, None, None,
                np, Spin::Unpolarized,
            ).unwrap();
            let auxes = vec![AuxiliaryConfig {
                alpha: 1.0,
                weight: 1.0,
                thresholds: default_thresholds(),
            }];
            evaluate_mixed_lda(&input, DerivativeOrder::Vxc, &mut out, &auxes, &mut ws).unwrap();
        }

        assert!(zk[0] < 0.0, "zk should be negative, got {}", zk[0]);
        assert!(vrho[0] != 0.0, "vrho should be non-zero, got {}", vrho[0]);
    }

    #[test]
    fn mixed_exc_order_with_vrho_none_no_panic() {
        let rho = vec![0.5, 1.0];
        let np = 2;
        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();

        let mut zk = vec![0.0f64; np];
        let mut ws = EvaluationWorkspace::new(np, Spin::Unpolarized);
        {
            let mut out = LdaOutput::new(
                Some(&mut zk), None, None, None, None,
                np, Spin::Unpolarized,
            ).unwrap();
            let auxes = vec![AuxiliaryConfig {
                alpha: 1.0,
                weight: 1.0,
                thresholds: default_thresholds(),
            }];
            // Should not panic even though vrho is None
            evaluate_mixed_lda(&input, DerivativeOrder::Exc, &mut out, &auxes, &mut ws).unwrap();
        }

        for i in 0..np {
            assert!(zk[i] < 0.0, "zk[{i}] should be negative");
        }
    }

    #[test]
    fn mixed_fxc_order_through_mixed_path() {
        let rho = vec![0.5];
        let np = 1;
        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();

        // Direct
        let mut zk_d = vec![0.0f64; np];
        let mut vrho_d = vec![0.0f64; np];
        let mut v2rho2_d = vec![0.0f64; np];
        {
            let mut out = LdaOutput::new(
                Some(&mut zk_d), Some(&mut vrho_d), Some(&mut v2rho2_d), None, None,
                np, Spin::Unpolarized,
            ).unwrap();
            dispatch_lda(&input, DerivativeOrder::Fxc, &mut out, 1.0, &default_thresholds()).unwrap();
        }

        // Mixed with weight=1.0
        let mut zk_m = vec![0.0f64; np];
        let mut vrho_m = vec![0.0f64; np];
        let mut v2rho2_m = vec![0.0f64; np];
        let mut ws = EvaluationWorkspace::new(np, Spin::Unpolarized);
        {
            let mut out = LdaOutput::new(
                Some(&mut zk_m), Some(&mut vrho_m), Some(&mut v2rho2_m), None, None,
                np, Spin::Unpolarized,
            ).unwrap();
            let auxes = vec![AuxiliaryConfig {
                alpha: 1.0,
                weight: 1.0,
                thresholds: default_thresholds(),
            }];
            evaluate_mixed_lda(&input, DerivativeOrder::Fxc, &mut out, &auxes, &mut ws).unwrap();
        }

        assert!((zk_m[0] - zk_d[0]).abs() < 1e-15);
        assert!((vrho_m[0] - vrho_d[0]).abs() < 1e-15);
        assert!((v2rho2_m[0] - v2rho2_d[0]).abs() < 1e-15);
    }
}
