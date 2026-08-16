//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta648 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2379;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2380;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2381;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2382;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2383;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2384;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2385;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2386;
use chunk8::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2387;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta648<F: Float>(t41656: F, t41658: F, t41660: F, t41662: F, t41675: F, t41678: F, t41680: F, t41682: F, t41684: F, t41713: F, t42245: F, t47744: F, t47748: F, t47761: F, t47765: F, t47769: F, t47777: F, t47781: F, t47785: F, t47787: F, t10813: F, t4433: F, t10743: F, t10765: F, t10771: F, t10805: F, t10811: F, t14266: F, t14328: F, t14432: F, t14435: F, t14436: F, t14442: F, t1569: F, t2861: F, t2862: F, t2880: F, t2881: F, t2886: F, t2888: F, t2889: F, t311: F, t41984: F, t42154: F, t47791: F, t48747: F, t48750: F, t48765: F, t48789: F, t48813: F, t931: F, t13716: F, t2932: F, t10632: F, t4471: F, t47681: F, t47686: F, t47691: F, t47695: F, t47699: F, t47703: F, t47705: F, t48085: F, t48087: F, t48090: F, t48092: F, t48096: F, t41831: F, t41833: F, t47707: F, t47709: F, t47711: F, t47713: F, t47715: F, t47717: F, t47722: F, t47724: F, t47728: F, t47730: F, t47732: F, t47736: F, t47738: F, t48098: F, t48101: F, t48103: F, t41904: F, t901: F, t41863: F, t41865: F, t41870: F, t41872: F, t41874: F, t41876: F, t48112: F, t48114: F, t48116: F, t48119: F, t48122: F, t48125: F, t48128: F, t48131: F, t10588: F, t4362: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t48833 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2379::<F>(t41656, t41658, t41660, t41662, t41675, t41678, t41680, t41682, t41684, t41713, t42245, t47744, t47748, t47761, t47765, t47769, t47777, t47781, t47785, t47787);
        let t48861 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2380::<F>(t10813, t4433, t10743, t10765, t10771, t10805, t10811, t14266, t14328, t14432, t14435, t14436, t14442, t1569, t2861, t2862, t2880, t2881, t2886, t2888, t2889, t311, t41984, t42154, t47791, t48747, t48750, t48765, t48789, t48813, t48833, t931);
        let (t48883, t48890, t48907) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2381::<F>(t13716, t2932, t10632, t4471, t47681, t47686, t47691, t47695, t47699, t47703, t47705, t48085, t48087, t48090, t48092);
        let t48920 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2382::<F>(t48096, t41831, t41833, t47707, t47709, t47711, t47713, t47715, t47717, t47722, t47724, t47728);
        let t48934 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2383::<F>(t47730, t41656, t41658, t41660, t47732, t47736, t47738, t47744, t47748, t48098, t48101, t48103);
        let t48960 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2384::<F>(t47705, t47707, t47730, t47681, t47686, t47691, t47695, t47699, t47703, t47709, t47711, t47713, t47715, t47717, t47722, t47724, t47728, t47732, t47736, t47738);
        let t48980 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2385::<F>(t41656, t41658, t41660, t41662, t41675, t41678, t41680, t41682, t41684, t41713, t41904, t47744, t47748, t47761, t47765, t47769, t47777, t47781, t47785, t47787);
        let (t48981, t48982, t48990) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2386::<F>(t48960, t48980, t901, t41662, t41675, t41678, t41682, t41684, t41863, t41865, t41870, t41872, t41874, t41876);
        let (t49004, t49009) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2387::<F>(t47761, t47765, t47769, t48112, t48114, t48116, t48119, t48122, t48125, t48128, t48131, t10588, t4362);
    (t48861, t48883, t48890, t48907, t48920, t48934, t48981, t48982, t48990, t49004, t49009)
}
