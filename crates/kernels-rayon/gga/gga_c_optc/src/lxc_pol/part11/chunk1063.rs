//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1063/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1063(t1278: f64, t6923: f64, t1274: f64, t22797: f64, t1261: f64, t22850: f64, t1339: f64, t3696: f64) -> (f64, f64, f64, f64) {
    let t29592 = t6923 * t1278;
    let t29750 = t22797 * t1274;
    let t29752 = t22850 * t1261;
    let t30189 = t3696 * t1339;
    (t29592, t29750, t29752, t30189)
}
