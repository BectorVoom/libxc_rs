//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 583/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk583(t951: f64, t2428: f64, t2363: f64, t410: f64, t2029: f64, t2368: f64, t2126: f64, t2370: f64, t914: f64, t937: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2429 = t951 * t951;
    let t2430 = t2428 * t2429;
    let t2433 = t2363 * t410;
    let t2434 = t2368 * t2029;
    let t2435 = t2126 * t2370;
    let t2436 = t2434 * t2435;
    let t2439 = t914 * t937;
    (t2429, t2430, t2433, t2434, t2436, t2439)
}
