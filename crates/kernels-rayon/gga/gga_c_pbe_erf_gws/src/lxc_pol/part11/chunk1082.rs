//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1082/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1082(t17321: f64, t17322: f64, t47377: f64, t639: f64, t11136: f64, t1621: f64, t3553: f64, t31168: f64, t3535: f64, t39931: f64, t3342: f64) -> (f64, f64, f64, f64, f64) {
    let t47381 = 352.0_f64 / 243.0_f64 * t639 * t17321 * t17322 * t47377;
    let t47385 = 24.0_f64 / 5.0_f64 * t639 * t1621 * t11136 * t3553;
    let t47387 = 16.0_f64 / 5.0_f64 * t31168 * t3535;
    let t47389 = 32.0_f64 / 45.0_f64 * t39931;
    let t47391 = t3342 * t3342;
    (t47381, t47385, t47387, t47389, t47391)
}
