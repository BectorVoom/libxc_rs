//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2494/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2494(t13265: f64, t46741: f64, t13258: f64, t13333: f64, t12985: f64, t9577: f64, t212: f64, t4119: f64, t2586: f64, t9523: f64, t4138: f64, t9541: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46742 = t46741 * t13265;
    let t46748 = t13258 * t13333;
    let t46764 = t9577 * t12985;
    let t46766 = t212 * t4119;
    let t46768 = t2586 * t9523 * t46766;
    let t46770 = t9541 * t4138;
    (t46742, t46748, t46764, t46766, t46768, t46770)
}
