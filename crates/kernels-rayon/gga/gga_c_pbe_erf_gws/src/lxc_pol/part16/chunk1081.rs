//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1081/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1081(t13872: f64, t3965: f64, t2307: f64, t3975: f64, t3972: f64, t1176: f64, t2333: f64, t1180: f64, t2397: f64, t3952: f64, t1178: f64, t2353: f64, t371: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13873 = t3965 * t13872;
    let t13877 = t3975 * t2307;
    let t13878 = t3972 * t13877;
    let t13893 = t1176 * t2333;
    let t13894 = t13893 * t1180;
    let t13896 = t3952 * t2397;
    let t13899 = t371 * t1178 * t2353;
    (t13873, t13877, t13878, t13893, t13894, t13896, t13899)
}
