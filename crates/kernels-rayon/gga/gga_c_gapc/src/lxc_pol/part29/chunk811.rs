//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 811/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk811(t7927: f64, t876: f64, t3378: f64, t3367: f64, t3383: f64, t3382: f64, t2660: f64, t9067: f64, t8135: f64, t1018: f64, t2619: f64, t2621: f64, t3096: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9429 = t7927 * t876;
    let t9430 = t3378 * t9429;
    let t9432 = t3367 * t3383;
    let t9433 = t3382 * t9432;
    let t9435 = t2660 * t9067;
    let t9436 = t9435 * t8135;
    let t9438 = t2619 * t1018;
    let t9439 = t3096 * t2621;
    (t9430, t9433, t9435, t9436, t9438, t9439)
}
