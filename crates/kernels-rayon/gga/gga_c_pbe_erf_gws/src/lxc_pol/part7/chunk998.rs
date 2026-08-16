//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 998/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk998(t1914: f64, t5421: f64, t17328: f64, t17330: f64, t17335: f64, t17338: f64, t17341: f64, t17343: f64, t18240: f64, t18243: f64, t18245: f64, t18247: f64, t18250: f64) -> f64 {
    let t18252 = t1914 * t5421;
    let t18254 = t17328 + t17330 - t17335 + t17338 + t17341 - t17343 + t18240 - t18243 - t18245 + 2.0_f64 * t18247 + 4.0_f64 / 3.0_f64 * t18250 + 0.72933333333333333331e0_f64 * t18252;
    t18254
}
