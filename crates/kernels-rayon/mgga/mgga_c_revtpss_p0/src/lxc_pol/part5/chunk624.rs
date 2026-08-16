//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 624/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk624(t3670: f64, t480: f64, t1236: f64, t127: f64, t371: f64, t1235: f64, t221: f64, t462: f64, t696: f64, t461: f64, t1226: f64, t140: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3671 = t3670 * t480;
    let t3678 = t371 * t127 * t1236;
    let t3679 = t1235 * t3678;
    let t3682 = t221 * t696 * t462;
    let t3684 = t461 * t3682 / 432.0_f64;
    let t3685 = t140 * t1226;
    (t3671, t3678, t3679, t3682, t3684, t3685)
}
