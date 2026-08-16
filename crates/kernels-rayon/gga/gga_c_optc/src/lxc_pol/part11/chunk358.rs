//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 358/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk358(t1514: f64, t1583: f64, t1582: f64, t1013: f64, t496: f64, t1011: f64, t429: f64) -> (f64, f64, f64, f64) {
    let t1584 = t1583 * t1514;
    let t1585 = t1582 * t1584;
    let t1587 = t1013 * t496;
    let t1588 = t1011 * t429 * t1587;
    (t1584, t1585, t1587, t1588)
}
