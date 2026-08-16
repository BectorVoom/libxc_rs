//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2105/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2105(t67: f64, t792: f64, t9558: f64, t133: f64, t1484: f64, t41214: f64, t6600: f64, t213: f64, t221: f64, t13004: f64, t782: f64, t131: f64, t205: f64, t41160: f64) -> (f64, f64, f64, f64, f64) {
    let t46799 = t792 * t9558 * t67;
    let t46806 = t41214 * t133 * t6600 * t1484;
    let t46838 = t221 * t213;
    let t46843 = t782 * t13004;
    let t46847 = t205 * t41160 * t131;
    (t46799, t46806, t46838, t46843, t46847)
}
