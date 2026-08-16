//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 726/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk726(t2953: f64, t8337: f64, t1004: f64, t1265: f64, t517: f64, t1007: f64, t2933: f64, t2948: f64, t2951: f64, t423: f64, t1459: f64, t2954: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8338 = t2953 * t8337;
    let t8340 = t1004 * t1265;
    let t8341 = t8340 * t517;
    let t8342 = t8341 * t1007;
    let t8344 = t2933 * t2948;
    let t8346 = t2951 * t423;
    let t8347 = t8346 * t1459;
    let t8348 = t8347 * t2954;
    (t8338, t8341, t8342, t8344, t8347, t8348)
}
