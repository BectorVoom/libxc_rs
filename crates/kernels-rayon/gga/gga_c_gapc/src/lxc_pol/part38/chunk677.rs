//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 677/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk677(t126: f64, t6939: f64, t102: f64, t786: f64, t2530: f64, t2207: f64, t2446: f64, t875: f64, t2614: f64, t442: f64, t2462: f64, t883: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6940 = t6939 * t126;
    let t6942 = t102 * t786;
    let t6943 = t2530 * t6942;
    let t6948 = t2207 * t126;
    let t6951 = t2446 * t102 * t875;
    let t7029 = t2614 * t442;
    let t7053 = t2462 * t883;
    (t6940, t6943, t6948, t6951, t7029, t7053)
}
