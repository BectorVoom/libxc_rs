//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta729 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2390;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2391;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2392;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2393;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2394;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2395;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta729(t48157: f64, t60192: f64, t60194: f64, t60202: f64, t68571: f64, t68577: f64, t68580: f64, t68583: f64, t68586: f64, t68589: f64, t68592: f64, t10564: f64, t123: f64, t68521: f64, t68525: f64, t47774: f64, t47775: f64, t68513: f64, t47779: f64, t47783: f64, t41959: f64, t59663: f64, t59665: f64, t59680: f64, t59688: f64, t59694: f64, t60204: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t68594, t68596) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2390(t48157, t60192, t60194, t60202, t68571, t68577, t68580, t68583, t68586, t68589, t68592, t10564, t123, t68521);
        let t68599 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2391(t10564, t123, t68525);
        let t68602 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2392(t47774, t47775, t68513);
        let t68605 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2393(t47774, t47779, t68513);
        let t68608 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2394(t47774, t47783, t68513);
        let t68616 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2395(t41959, t59663, t59665, t59680, t59688, t59694, t60204, t68596, t68599, t68602, t68605, t68608);
    (t68594, t68596, t68599, t68602, t68605, t68608, t68616)
}
