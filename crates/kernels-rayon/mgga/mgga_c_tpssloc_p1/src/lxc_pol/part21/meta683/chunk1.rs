//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2496/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2496(t2559: f64, t4126: f64, t4130: f64, t12997: f64, t13000: f64, t2566: f64, t67: f64, t792: f64, t9558: f64, t12984: f64, t2379: f64, t686: f64) -> (f64, f64, f64, f64) {
    let t46793 = t2559 * t4126 * t4130;
    let t46796 = t2566 * t12997 * t13000;
    let t46799 = t792 * t9558 * t67;
    let t46802 = t46799 * t686 * t12984 * t2379;
    (t46793, t46796, t46799, t46802)
}
