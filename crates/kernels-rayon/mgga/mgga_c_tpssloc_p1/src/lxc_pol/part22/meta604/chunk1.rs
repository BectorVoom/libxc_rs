//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2127/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2127(t49690: f64, t14202: f64, t3117: f64, t10890: f64, t14507: f64, t3038: f64, t49650: f64, t1020: f64, t10508: f64, t248: f64, t4650: f64, t13965: f64, t3109: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t49691 = t49690 / 4608.0_f64;
    let t49692 = t3117 * t14202;
    let t49693 = t49692 / 6912.0_f64;
    let t49743 = t14507 * t10890;
    let t49771 = t49650 * t3038;
    let t49818 = t1020 * t248 * t10508 * t4650;
    let t49819 = t49818 / 4608.0_f64;
    let t49831 = t3109 * t13965;
    (t49691, t49693, t49743, t49771, t49819, t49831)
}
