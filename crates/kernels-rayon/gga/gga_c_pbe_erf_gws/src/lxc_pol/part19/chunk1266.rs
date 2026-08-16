//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1266/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1266(t53975: f64, t53985: f64, t54429: f64, t4227: f64, t6781: f64, t829: f64, t830: f64, t14886: f64, t4386: f64, t892: f64, t15036: f64, t19906: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t55741 = 7.0_f64 / 288.0_f64 * t53975;
    let t55745 = 7.0_f64 / 36.0_f64 * t53985;
    let t55752 = 7.0_f64 / 72.0_f64 * t54429;
    let t55762 = t6781 * t4227;
    let t55764 = t829 * t830 * t55762;
    let t55769 = t4386 * t892 * t14886;
    let t55773 = 7.0_f64 / 72.0_f64 * t19906 * t15036;
    (t55741, t55745, t55752, t55764, t55769, t55773)
}
