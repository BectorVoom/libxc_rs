//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 987/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk987(t3247: f64, t460: f64, t2244: f64, t1176: f64, t134: f64, t1184: f64, t3451: f64, t3447: f64, t3448: f64, t3475: f64, t1239: f64, t68: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11583 = t460 * t3247;
    let t11584 = t11583 * t2244;
    let t11588 = t134 * t1176;
    let t11589 = t11588 * t1184;
    let t11590 = t11589 * t3451;
    let t11591 = t3447 * t11590;
    let t11593 = t3448 * t3475;
    let t11604 = t1239 * t1239;
    let t11605 = 1.0_f64 / t11604;
    let t11606 = t68 * t11605;
    (t11583, t11584, t11588, t11589, t11591, t11593, t11606)
}
