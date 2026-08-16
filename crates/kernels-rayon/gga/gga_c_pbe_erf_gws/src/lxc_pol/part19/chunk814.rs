//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 814/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk814(t34: f64, t649: f64, t641: f64, t837: f64, t2592: f64, t639: f64, t2597: f64, t5493: f64, t1620: f64, t1627: f64, t2640: f64, t1660: f64, t197: f64) -> (f64, f64, f64, f64, f64) {
    let t7019 = t649 * t34;
    let t7039 = t837 * t641;
    let t7040 = t7039 * t2592;
    let t7041 = t639 * t7040;
    let t7043 = t5493 * t2597;
    let t7045 = 16.0_f64 / 45.0_f64 * t1620 * t7043;
    let t7047 = 16.0_f64 / 135.0_f64 * t1627 * t2640;
    let t7048 = t1660 * t197;
    (t7019, t7041, t7045, t7047, t7048)
}
