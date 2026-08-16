//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 415/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk415(t2153: f64, t282: f64, t61: f64, t268: f64, t995: f64, t19: f64, t792: f64, t1561: f64, t315: f64, t277: f64, t825: f64, t1474: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2154 = t2153 * t282;
    let t2155 = t61 * t2154;
    let t2158 = t995 * t268;
    let t2159 = t792 * t19;
    let t2160 = t2158 * t2159;
    let t2161 = t1561 * t315;
    let t2164 = t277 * t825;
    let t2165 = t1474 * t2164;
    (t2155, t2158, t2160, t2161, t2164, t2165)
}
