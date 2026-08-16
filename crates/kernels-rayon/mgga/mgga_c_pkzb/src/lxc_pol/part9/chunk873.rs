//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 873/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk873(t404: f64, t6382: f64, t2185: f64, t2411: f64, t824: f64, t758: f64, t179: f64, t2405: f64, t6106: f64, t932: f64, t53: f64, t2226: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6383 = t404 * t6382;
    let t6386 = t2411 * t824 * t2185;
    let t6387 = t758 * t6386;
    let t6391 = t179 * t2405 * t2185;
    let t6392 = t404 * t6391;
    let t6395 = t179 * t932 * t6106;
    let t6398 = t53 * t2411;
    let t6400 = t179 * t6398 * t2226;
    (t6383, t6386, t6387, t6391, t6392, t6395, t6400)
}
