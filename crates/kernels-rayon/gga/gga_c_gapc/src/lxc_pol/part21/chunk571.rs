//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 571/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk571(t3320: f64, t3322: f64, t1084: f64, t3138: f64, t291: f64, t959: f64) -> (f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t3323 = t3320 * t3322;
    let t3325 = t1084 * t3138;
    let t3326 = pi * t291;
    let t3327 = t3326 * t959;
    (t3323, t3325, t3326, t3327)
}
