//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta660 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2203;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2204;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta660(t16558: f64, t707: f64, t751: f64, t16586: f64, t9929: f64, t16579: f64, t172: f64, t763: f64, t67: f64, t758: f64, t16957: f64, t41011: f64, t212: f64, t5544: f64, t12998: f64, t686: f64, t776: f64, t13012: f64, t16798: f64, t16773: f64, t46843: f64, t16777: f64, t5527: f64, t46799: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t59037, t59039, t59045, t59048, t59100) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2203(t16558, t707, t751, t16586, t9929, t16579, t172, t763, t67, t758, t16957, t41011);
        let (t59135, t59138, t59140, t59154, t59156, t59162, t59165) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2204(t212, t5544, t12998, t686, t776, t13012, t16798, t16773, t46843, t16777, t5527, t46799);
    (t59037, t59039, t59045, t59048, t59100, t59135, t59138, t59140, t59154, t59156, t59162, t59165)
}
