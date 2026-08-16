//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta635 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2330;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2331;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2332;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2333;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2334;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2335;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2336;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta635<F: Float>(t118: F, t122: F, t154: F, t10277: F, t1043: F, t45971: F, t10216: F, t3061: F, t2770: F, t376: F, t1540: F, t9698: F, t41656: F, t41658: F, t41660: F, t41662: F, t41675: F, t41678: F, t41680: F, t41682: F, t41684: F, t41713: F, t41741: F, t47744: F, t47748: F, t47761: F, t47765: F, t47769: F, t324: F, t47740: F, t300: F, t1589: F, t42281: F, t10696: F, t2842: F, t4399: F, t10662: F, t1556: F, t42100: F, t42102: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t47774 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2330::<F>(t118, t122, t154);
        let (t47775, t47777) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2331::<F>(t10277, t1043, t45971, t47774);
        let (t47779, t47781) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2332::<F>(t10216, t3061, t45971, t47774);
        let t47785 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2333::<F>(t2770, t376, t45971, t47774);
        let t47787 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2334::<F>(t1540, t9698);
        let t47789 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2335::<F>(t41656, t41658, t41660, t41662, t41675, t41678, t41680, t41682, t41684, t41713, t41741, t47744, t47748, t47761, t47765, t47769, t47777, t47781, t47785, t47787);
        let (t47791, t47793, t47795, t47798, t47802) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2336::<F>(t324, t47740, t47789, t300, t1589, t42281, t10696, t2842, t4399, t10662, t1556, t42100, t42102);
    (t47774, t47775, t47777, t47779, t47781, t47785, t47787, t47791, t47793, t47795, t47798, t47802)
}
