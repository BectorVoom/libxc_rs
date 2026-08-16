//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 957/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk957(t722: f64, t7474: f64, t2826: f64, t713: f64, t1070: f64, t1854: f64, t1857: f64, t1088: f64, t1915: f64, t2743: f64, t663: f64, t685: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7475 = t7474 * t722;
    let t7478 = t2826 * t713;
    let t7483 = t1070 * t1854;
    let t7485 = 2.0_f64 * t7483 * t1857;
    let t7486 = t1088 * t1915;
    let t7489 = t2743 * t663;
    let t7491 = 2.0_f64 * t7489 * t685;
    (t7475, t7478, t7483, t7485, t7486, t7489, t7491)
}
