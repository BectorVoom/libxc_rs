//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1074/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1074(t16490: f64, t18149: f64, t18155: f64, t42943: f64, t42948: f64, t47293: f64, t47297: f64, t47299: f64, t47301: f64, t47303: f64, t47307: f64, t12339: f64, t1820: f64, t1821: f64, t7899: f64) -> (f64, f64) {
    let t47308 = t18149 + 4.0_f64 / 3.0_f64 * t42943 - t18155 + 0.24311111111111111111e0_f64 * t42948 - t16490 + t47293 + t47297 + t47299 + t47301 - t47303 + t47307;
    let t47315 = 64.0_f64 / 15.0_f64 * t1820 * t1821 * t7899 * t12339;
    (t47308, t47315)
}
