//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2261/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2261(t12984: f64, t12998: f64, t2553: f64, t686: f64, t12990: f64, t13012: f64, t12994: f64, t213: f64, t221: f64, t13196: f64, t776: f64, t13004: f64, t782: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46828 = t12998 * t686 * t12984 * t2553;
    let t46830 = t13012 * t12990;
    let t46836 = t13012 * t12994;
    let t46838 = t221 * t213;
    let t46839 = t13196 * t776;
    let t46843 = t782 * t13004;
    (t46828, t46830, t46836, t46838, t46839, t46843)
}
