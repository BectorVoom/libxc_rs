//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1147/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1147(t17045: f64, t2641: f64, t16816: f64, t2472: f64, t16671: f64, t241: f64, t16770: f64, t2520: f64, t2476: f64, t828: f64, t16699: f64, t809: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t49900 = t2641 * t17045;
    let t49939 = t2472 * t16816;
    let t49995 = t241 * t16671;
    let t50450 = t16770 * t2520;
    let t50490 = t16816 * t2476;
    let t50563 = t16671 * t828;
    let t50691 = t16699 * t809;
    (t49900, t49939, t49995, t50450, t50490, t50563, t50691)
}
