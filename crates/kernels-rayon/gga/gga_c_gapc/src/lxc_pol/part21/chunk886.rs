//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 886/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk886(t10110: f64, t3247: f64, t277: f64, t6851: f64, t8449: f64, t7108: f64, t8452: f64, t959: f64, t3253: f64, t6940: f64, t2438: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10284 = t10110 * t3247;
    let t10286 = t277 * t6851;
    let t10287 = t8449 * t10286;
    let t10289 = t8452 * t959 * t7108;
    let t10290 = t10287 * t10289;
    let t10292 = t3253 * t6940;
    let t10293 = t2438 * t959;
    (t10284, t10286, t10287, t10290, t10292, t10293)
}
