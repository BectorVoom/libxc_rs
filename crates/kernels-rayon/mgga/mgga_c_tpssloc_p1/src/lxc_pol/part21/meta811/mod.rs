//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta811 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2843;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2844;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2845;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2846;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2847;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2848;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2849;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta811(t17156: f64, t2250: f64, t123: f64, t2768: f64, t12606: f64, t4337: f64, t41741: f64, t59688: f64, t59692: f64, t59694: f64, t59698: f64, t59700: f64, t59702: f64, t59704: f64, t59708: f64, t59713: f64, t10216: f64, t2244: f64, t5398: f64, t10564: f64, t16558: f64, t2775: f64, t607: f64, t882: f64, t47774: f64, t47779: f64, t55716: f64, t47775: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t59715, t59717) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2843(t17156, t2250, t123, t2768);
        let (t59719, t59721) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2844(t12606, t4337, t123, t2768);
        let t59723 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2845(t41741, t59688, t59692, t59694, t59698, t59700, t59702, t59704, t59708, t59713, t59717, t59721);
        let (t59725, t59727) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2846(t10216, t2244, t5398, t10564, t123);
        let (t59730, t59732) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2847(t16558, t2775, t607, t123, t882);
        let t59735 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2848(t47774, t47779, t55716);
        let t59738 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2849(t47774, t47775, t55716);
    (t59715, t59717, t59719, t59721, t59723, t59725, t59727, t59730, t59732, t59735, t59738)
}
