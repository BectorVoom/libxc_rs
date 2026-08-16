//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 771/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk771(t7289: f64, t7292: f64, t123: f64, t2101: f64, t5263: f64, t883: f64, t943: f64, t161: f64, t2610: f64, t2095: f64, t2581: f64, t5397: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7293 = t7289 * t7292;
    let t7296 = t2101 * t123;
    let t7297 = t883 * t5263;
    let t7298 = t7296 * t7297;
    let t7299 = t943 * t7298;
    let t7301 = t161 * t2610;
    let t7302 = t2095 * t7301;
    let t7303 = t943 * t7302;
    let t7305 = t2581 * t5397;
    (t7293, t7297, t7299, t7301, t7303, t7305)
}
