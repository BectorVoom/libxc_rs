//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1125/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1125(t32019: f64, t3513: f64, t12440: f64, t7527: f64, t12444: f64, t1044: f64, t1620: f64, t1621: f64, t40687: f64, t18280: f64, t47902: f64, t47904: f64, t47906: f64, t47910: f64, t47914: f64, t47916: f64) -> (f64, f64, f64, f64, f64) {
    let t47918 = 16.0_f64 / 5.0_f64 * t32019 * t3513;
    let t47920 = 16.0_f64 / 5.0_f64 * t7527 * t12440;
    let t47922 = 16.0_f64 / 5.0_f64 * t7527 * t12444;
    let t47926 = 16.0_f64 / 15.0_f64 * t1620 * t1621 * t40687 * t1044;
    let t47927 = -t47902 - t47904 - t47906 + t18280 - t47910 + t47914 + t47916 - t47918 - t47920 - t47922 - t47926;
    (t47918, t47920, t47922, t47926, t47927)
}
