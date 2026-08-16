//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2970/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2970(t13981: f64, t9962: f64, t13951: f64, t2713: f64, t3964: f64, t1413: f64, t46835: f64, t48698: f64, t13845: f64, t13847: f64, t13848: f64, t4004: f64) -> (f64, f64, f64, f64) {
    let t49005 = t9962 * t13981;
    let t49008 = t3964 * t2713 * t13951;
    let t49012 = t46835 * t1413 * t48698;
    let t49016 = t13845 * t13847 * t13848 * t4004;
    (t49005, t49008, t49012, t49016)
}
