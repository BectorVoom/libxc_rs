//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1086/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1086(t13888: f64, t2410: f64, t9283: f64, t1176: f64, t2333: f64, t1180: f64, t2397: f64, t3952: f64, t1178: f64, t2353: f64, t371: f64, t1177: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13889 = t13888 * t2410;
    let t13890 = t9283 * t13889;
    let t13893 = t1176 * t2333;
    let t13894 = t13893 * t1180;
    let t13895 = 119.0_f64 / 13824.0_f64 * t13894;
    let t13896 = t3952 * t2397;
    let t13899 = t371 * t1178 * t2353;
    let t13900 = t1177 * t13899;
    (t13889, t13890, t13893, t13895, t13896, t13899, t13900)
}
