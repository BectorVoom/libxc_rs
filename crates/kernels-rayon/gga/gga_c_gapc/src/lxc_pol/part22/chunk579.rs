//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 579/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk579(t3325: f64, t3330: f64, t2660: f64, t3045: f64, t2767: f64, t129: f64, t2619: f64, t197: f64, t2621: f64, t2712: f64, t1077: f64, t1936: f64, t916: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3331 = t3325 * t3330;
    let t3333 = t2660 * t3045;
    let t3334 = t3333 * t2767;
    let t3336 = t2619 * t129;
    let t3337 = t197 * t2621;
    let t3338 = t3336 * t3337;
    let t3340 = t197 * t2712;
    let t3341 = t1077 * t3340;
    let t3343 = t916 * t1936;
    (t3331, t3334, t3336, t3337, t3338, t3340, t3341, t3343)
}
