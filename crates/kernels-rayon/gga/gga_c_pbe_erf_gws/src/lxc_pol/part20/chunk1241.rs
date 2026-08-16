//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1241/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1241(t14469: f64, t50943: f64, t13793: f64, t53229: f64, t3165: f64, t898: f64, t14456: f64, t51666: f64, t1114: f64, t51916: f64, t50935: f64, t1112: f64, t2306: f64, t3074: f64, t833: f64, t837: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t53508 = t50943 * t14469;
    let t53509 = 7.0_f64 / 72.0_f64 * t53508;
    let t53515 = t53229 * t13793;
    let t53516 = 7.0_f64 / 72.0_f64 * t53515;
    let t53539 = t898 * t3165;
    let t53545 = t51666 * t14456;
    let t53546 = 7.0_f64 / 576.0_f64 * t53545;
    let t53566 = t1114 * t51916;
    let t53571 = t1114 * t50935;
    let t53577 = t3074 * t2306 * t1112 * t837 * t833;
    (t53509, t53516, t53539, t53546, t53566, t53571, t53577)
}
