//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1225/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1225(t2435: f64, t25352: f64, t11015: f64, t7018: f64, t7048: f64, t822: f64, t25300: f64, t9285: f64, t25299: f64, t7059: f64, t9288: f64, t7064: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t92858 = t2435 * t25352;
    let t92861 = 0.30356481678079769392e-1_f64 * t7018 * t11015;
    let t92864 = t822 * t7048;
    let t92868 = t25300 * t9285;
    let t92870 = 0.68540937416128198417e-2_f64 * t25299 * t92868;
    let t92871 = t7059 * t9288;
    let t92873 = 0.39982213492741449076e-1_f64 * t7064 * t92871;
    (t92858, t92861, t92864, t92868, t92870, t92871, t92873)
}
