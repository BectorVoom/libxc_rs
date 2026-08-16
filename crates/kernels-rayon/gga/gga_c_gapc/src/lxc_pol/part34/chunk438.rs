//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 438/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk438(t159: f64, t2141: f64, t104: f64, t751: f64, t14: f64, t260: f64, t445: f64, t348: f64, t19: f64, t269: f64, t1355: f64, t257: f64) -> (f64, f64, f64, f64, f64) {
    let t2346 = t2141 * t159;
    let t2349 = t751 * t104;
    let t2350 = t2349 * t14;
    let t2355 = t260 * t445;
    let t2356 = t2355 * t348;
    let t2357 = t269 * t19;
    let t2358 = t2357 * t1355;
    let t2361 = t14 * t257;
    (t2346, t2350, t2356, t2358, t2361)
}
