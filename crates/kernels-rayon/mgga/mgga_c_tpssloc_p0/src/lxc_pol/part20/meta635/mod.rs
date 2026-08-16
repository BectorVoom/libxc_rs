//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta635 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2330;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2331;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2332;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2333;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2334;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2335;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2336;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta635(t118: f64, t122: f64, t154: f64, t10277: f64, t1043: f64, t45971: f64, t10216: f64, t3061: f64, t2770: f64, t376: f64, t1540: f64, t9698: f64, t41656: f64, t41658: f64, t41660: f64, t41662: f64, t41675: f64, t41678: f64, t41680: f64, t41682: f64, t41684: f64, t41713: f64, t41741: f64, t47744: f64, t47748: f64, t47761: f64, t47765: f64, t47769: f64, t324: f64, t47740: f64, t300: f64, t1589: f64, t42281: f64, t10696: f64, t2842: f64, t4399: f64, t10662: f64, t1556: f64, t42100: f64, t42102: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t47774 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2330(t118, t122, t154);
        let (t47775, t47777) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2331(t10277, t1043, t45971, t47774);
        let (t47779, t47781) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2332(t10216, t3061, t45971, t47774);
        let t47785 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2333(t2770, t376, t45971, t47774);
        let t47787 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2334(t1540, t9698);
        let t47789 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2335(t41656, t41658, t41660, t41662, t41675, t41678, t41680, t41682, t41684, t41713, t41741, t47744, t47748, t47761, t47765, t47769, t47777, t47781, t47785, t47787);
        let (t47791, t47793, t47795, t47798, t47802) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2336(t324, t47740, t47789, t300, t1589, t42281, t10696, t2842, t4399, t10662, t1556, t42100, t42102);
    (t47774, t47775, t47777, t47779, t47781, t47785, t47787, t47791, t47793, t47795, t47798, t47802)
}
