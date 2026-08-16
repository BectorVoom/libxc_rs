//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1040/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1040(t1235: f64, t3678: f64, t221: f64, t462: f64, t696: f64, t461: f64, t1226: f64, t140: f64, t1222: f64, t1225: f64, t2258: f64, t1012: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3679 = t1235 * t3678;
    let t3682 = t221 * t696 * t462;
    let t3684 = t461 * t3682 / 432.0_f64;
    let t3685 = t140 * t1226;
    let t3686 = t1222 * t3685;
    let t3688 = t1225 * t2258;
    let t3689 = t1012 * t3688;
    (t3679, t3682, t3684, t3685, t3686, t3688, t3689)
}
