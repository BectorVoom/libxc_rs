//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 100/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk100(t22: f64, t268: f64, t159: f64, t260: f64, t106: f64, t269: f64, t103: f64, t164: f64, t266: f64, t276: f64) -> (f64, f64, f64, f64, f64) {
    let t299 = t22 * t268;
    let t303 = t260 * t159;
    let t304 = t106 * t269;
    let t310 = 0.58998125e-2_f64 * t303 * t304 - 0.21511666666666666667e-1_f64 * t103 * t164 * t266;
    let t311 = t310 * t276;
    (t299, t303, t304, t310, t311)
}
