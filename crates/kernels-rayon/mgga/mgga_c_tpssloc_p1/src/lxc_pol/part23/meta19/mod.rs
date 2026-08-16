//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta19 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk146;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk147;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk148;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk149;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk150;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk151;
use chunk6::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk152;
use chunk7::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk153;
use chunk8::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk154;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta19(t362: f64, t363: f64, t34: f64, t35: f64, rho0: f64, t354: f64, t335: f64, t67: f64, t246: f64, t120: f64, t61: f64, t283: f64, t339: f64, t350: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t364, t368) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk146(t362, t363, t34, t35, rho0);
        let t369 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk147(t364, t368);
        let t370 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk148(t354, t369);
        let t371 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk149(t335);
        let t372 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk150(t371);
        let t374 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk151(t372, t67, t246);
        let (t375, t376) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk152(t120, t61, t283);
        let t378 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk153(t374, t375, t376);
        let t381 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk154(t339, t350, t370, t378);
    (t364, t368, t369, t370, t371, t372, t374, t375, t376, t378, t381)
}
