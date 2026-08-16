//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 693/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk693(t252: f64, t828: f64, t232: f64, t6646: f64, t1888: f64, t1894: f64, t852: f64, t214: f64, t1880: f64, t1902: f64, t814: f64, t829: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6647 = t252 * t828;
    let t6648 = t6647 * t232;
    let t6649 = t6646 * t6648;
    let t6650 = t1888 * t6649;
    let t6652 = t1894 * t852;
    let t6653 = t214 * t6652;
    let t6654 = t1880 * t6653;
    let t6657 = t814 * t1902;
    let t6658 = t6657 * t829;
    (t6648, t6649, t6650, t6652, t6653, t6654, t6657, t6658)
}
