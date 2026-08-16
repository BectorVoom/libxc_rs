//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 957/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk957(t123: f64, t173: f64, t186: f64, t2537: f64, t2548: f64, t2554: f64, t2556: f64, t2597: f64, t2604: f64, t729: f64, t731: f64, t739: f64, t9291: f64, t9394: f64, t9485: f64, t9488: f64, t9501: f64, t9508: f64, t9514: f64, t9517: f64, t9521: f64, t9524: f64, t9525: f64, t9530: f64, t9533: f64, t9536: f64, t9537: f64, t9542: f64) -> f64 {
    let t9543 = 0.5848223622634646207e0_f64 * t739 * t9485 + 0.35089341735807877242e1_f64 * t2604 * t9488 + 0.16562821945185185185e-2_f64 * t123 * t9291 * t173 - 6.0_f64 * t2537 * t731 * t2548 + 0.96491876992155210402e2_f64 * t2554 * t2548 * t2556 * t729 - 0.35089341735807877242e1_f64 * t2597 * t9501 + 0.56968947174242584612e-3_f64 * t123 * t9291 * t186 + 0.51947577317044391277e2_f64 * t2604 * t9508 - t9394 - t9514 + t9517 + t9521 + t9524 + 6.0_f64 * t2554 * t9525 + 0.2069040516770936012e4_f64 * t9530 * t9533 + 0.10254018858216406658e4_f64 * t9536 * t9537 - t9542;
    t9543
}
