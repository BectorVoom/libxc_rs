//! MGGA_X_TAU_HCTH exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_tau_hcth.c`
//! by tools/translate_rayon/from_maple.py, then rewritten to
//! `wide::f64x8` by simd.py. Eight grid points per step; every lane runs maple2c's expression
//! sequence in its original order.
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]
use libxc_rkernel_math::constants::*;
use libxc_rkernel_math::simd;
use libxc_rkernel_math::wide::{f64x8, CmpEq, CmpGe, CmpGt, CmpLe, CmpLt, CmpNe};

const V_ZERO: f64x8 = f64x8::new([0.0; 8]);
const V_ONE: f64x8 = f64x8::new([1.0; 8]);

// Transcendentals in exact mode come from `libxc_rkernel_math::simd`,
// which is bit-identical / correctly-rounded per lane to the scalar calls
// the scalar kernel makes. In exact mode, the SIMD kernel produces output
// bit-identical to its scalar form.

/// Load 8 consecutive grid points.
///
/// The tail is padded by repeating the last element, not by zero-filling:
/// these formulas divide by rho, so a zero lane would raise inf/NaN in lanes
/// whose results are then discarded -- harmless to the answer, but it makes
/// any real NaN impossible to spot while debugging.
#[inline(always)]
fn load(s: &[f64], ip: usize, np: usize) -> f64x8 {
    if ip + 8 <= np {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        f64x8::new(b)
    } else {
        let mut b = [s[np - 1]; 8];
        b[..np - ip].copy_from_slice(&s[ip..np]);
        f64x8::new(b)
    }
}

/// Accumulate 8 consecutive grid points into an output array.
///
/// `+=`, not `=`. The scalar kernel writes `out[ip] += v`; a plain store is a
/// different operation in two ways. It keeps the sign of a negative zero where
/// `0.0 + -0.0` gives `+0.0` -- a bit difference the fingerprint gate reports
/// as a rejection even though no value changed (`gga_x_pbepow fxc` was
/// rejected on exactly this, 273 of 200,000 `v2sigma2` elements) -- and it
/// would discard whatever a caller had already put in the buffer.
#[inline(always)]
fn store_add(s: &mut [f64], ip: usize, m: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        let r: [f64; 8] = (f64x8::new(b) + acc).into();
        s[ip..ip + 8].copy_from_slice(&r);
    } else {
        for k in 0..m {
            s[ip + k] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_tau_hcth_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_cx_local_1: f64,
    param_cx_local_2: f64,
    param_cx_local_3: f64,
    param_cx_nlocal_1: f64,
    param_cx_nlocal_2: f64,
    param_cx_nlocal_3: f64,
    param_cx_nlocal_0: f64,
    param_cx_local_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_cx_local_1 = f64x8::splat(param_cx_local_1);
    let param_cx_local_2 = f64x8::splat(param_cx_local_2);
    let param_cx_local_3 = f64x8::splat(param_cx_local_3);
    let param_cx_nlocal_1 = f64x8::splat(param_cx_nlocal_1);
    let param_cx_nlocal_2 = f64x8::splat(param_cx_nlocal_2);
    let param_cx_nlocal_3 = f64x8::splat(param_cx_nlocal_3);
    let param_cx_nlocal_0 = f64x8::splat(param_cx_nlocal_0);
    let param_cx_local_0 = f64x8::splat(param_cx_local_0);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let v_lapl = load(lapl, ip, np);
        let v_tau = load(tau, ip, np);
        let mut acc_zk = V_ZERO;
        {
            let t3 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t4 = f64x8::splat(M_CBRT3);
            let t5 = f64x8::splat(M_CBRTPI);
            let t7 = t4 / t5;
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t14 = (simd::cbrt(zeta_threshold));
            let t16 = (simd::cbrt(t12));
            let t18 = (((t12).simd_le(zeta_threshold)).select(t14 * zeta_threshold, t16 * t12));
            let t19 = (simd::cbrt(v_rho));
            let t20 = t18 * t19;
            let t22 = param_cx_local_1;
            let t23 = t22 * v_sigma;
            let t24 = f64x8::splat(M_CBRT2);
            let t25 = t24 * t24;
            let t26 = v_rho * v_rho;
            let t27 = t19 * t19;
            let t29 = f64x8::splat(1.0) / t27 / t26;
            let t30 = t25 * t29;
            let t34 = f64x8::splat(1.0) + f64x8::splat(0.004) * v_sigma * t25 * t29;
            let t35 = f64x8::splat(1.0) / t34;
            let t36 = t30 * t35;
            let t39 = param_cx_local_2;
            let t40 = v_sigma * v_sigma;
            let t41 = t39 * t40;
            let t42 = t26 * t26;
            let t43 = t42 * v_rho;
            let t45 = f64x8::splat(1.0) / t19 / t43;
            let t46 = t24 * t45;
            let t47 = t34 * t34;
            let t48 = f64x8::splat(1.0) / t47;
            let t49 = t46 * t48;
            let t52 = param_cx_local_3;
            let t53 = t40 * v_sigma;
            let t54 = t52 * t53;
            let t55 = t42 * t42;
            let t56 = f64x8::splat(1.0) / t55;
            let t57 = t47 * t34;
            let t58 = f64x8::splat(1.0) / t57;
            let t59 = t56 * t58;
            let t63 = param_cx_nlocal_1;
            let t64 = t63 * v_sigma;
            let t67 = param_cx_nlocal_2;
            let t68 = t67 * t40;
            let t71 = param_cx_nlocal_3;
            let t72 = t71 * t53;
            let t75 = param_cx_nlocal_0 + f64x8::splat(0.004) * t64 * t36 + f64x8::splat(3.2e-05) * t68 * t49 + f64x8::splat(2.56e-07) * t72 * t59;
            let t76 = f64x8::splat(M_CBRT6);
            let t77 = t76 * t76;
            let t78 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t79 = (simd::cbrt(t78));
            let t80 = t79 * t79;
            let t82 = f64x8::splat(3.0) / f64x8::splat(10.0) * t77 * t80;
            let t83 = v_tau * t25;
            let t85 = f64x8::splat(1.0) / t27 / v_rho;
            let t86 = t83 * t85;
            let t87 = t82 - t86;
            let t88 = t82 + t86;
            let t89 = f64x8::splat(1.0) / t88;
            let t91 = t87 * t87;
            let t92 = t91 * t87;
            let t93 = t88 * t88;
            let t94 = t93 * t88;
            let t95 = f64x8::splat(1.0) / t94;
            let t98 = t91 * t91;
            let t99 = t98 * t87;
            let t100 = t93 * t93;
            let t102 = f64x8::splat(1.0) / t100 / t88;
            let t104 = t99 * t102 + t87 * t89 - f64x8::splat(2.0) * t92 * t95;
            let t106 = param_cx_local_0 + f64x8::splat(0.004) * t23 * t36 + f64x8::splat(3.2e-05) * t41 * t49 + f64x8::splat(2.56e-07) * t54 * t59 + t75 * t104;
            let t110 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t106));
            let tzk0 = f64x8::splat(2.0) * t110;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
