//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 509/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk509(t2099: f64, t762: f64, t757: f64, t178: f64, t755: f64, t752: f64) -> (f64, f64, f64) {
    let t2100 = t2099 * t762;
    let t2101 = t757 * t2100;
    let t2103 = t755 * t178;
    let t2104 = t752 * t2103;
    (t2100, t2101, t2104)
}
