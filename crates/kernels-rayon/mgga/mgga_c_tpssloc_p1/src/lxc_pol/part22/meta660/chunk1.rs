//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2204/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2204(t212: f64, t5544: f64, t12998: f64, t686: f64, t776: f64, t13012: f64, t16798: f64, t16773: f64, t46843: f64, t16777: f64, t5527: f64, t46799: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t59135 = t212 * t5544;
    let t59138 = t12998 * t686 * t59135 * t776;
    let t59140 = t13012 * t16798;
    let t59154 = t46843 * t16773;
    let t59156 = t13012 * t16777;
    let t59162 = t212 * t5527;
    let t59165 = t46799 * t686 * t59162 * t776;
    (t59135, t59138, t59140, t59154, t59156, t59162, t59165)
}
