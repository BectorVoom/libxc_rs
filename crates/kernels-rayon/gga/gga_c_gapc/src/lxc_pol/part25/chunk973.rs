//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 973/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk973(t11320: f64, t611: f64, t1720: f64, t8950: f64, t3137: f64, t519: f64) -> (f64, f64, f64, f64) {
    let t11321 = t611 * t11320;
    let t11322 = t1720 * t8950;
    let t11323 = t11321 * t11322;
    let t11325 = t519 * t3137;
    (t11321, t11322, t11323, t11325)
}
