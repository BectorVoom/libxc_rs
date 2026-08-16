//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 657/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk657(t125: f64, t1558: f64, t1544: f64, t854: f64, t236: f64, t807: f64, t1469: f64, t2375: f64, t2382: f64, t1532: f64, t750: f64, t1534: f64, t177: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4365 = t125 * t1558;
    let t4371 = t854 * t1544;
    let t4372 = t236 * t4371;
    let t4373 = t807 * t4372;
    let t4377 = t2375 * t1469;
    let t4384 = t2382 * t1469;
    let t4397 = t1532 * t750;
    let t4398 = t1534 * t177;
    (t4365, t4371, t4372, t4373, t4377, t4384, t4397, t4398)
}
