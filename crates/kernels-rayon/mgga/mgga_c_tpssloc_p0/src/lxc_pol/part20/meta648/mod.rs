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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta648(t41656: f64, t41658: f64, t41660: f64, t41662: f64, t41675: f64, t41678: f64, t41680: f64, t41682: f64, t41684: f64, t41713: f64, t42245: f64, t47744: f64, t47748: f64, t47761: f64, t47765: f64, t47769: f64, t47777: f64, t47781: f64, t47785: f64, t47787: f64, t10813: f64, t4433: f64, t10743: f64, t10765: f64, t10771: f64, t10805: f64, t10811: f64, t14266: f64, t14328: f64, t14432: f64, t14435: f64, t14436: f64, t14442: f64, t1569: f64, t2861: f64, t2862: f64, t2880: f64, t2881: f64, t2886: f64, t2888: f64, t2889: f64, t311: f64, t41984: f64, t42154: f64, t47791: f64, t48747: f64, t48750: f64, t48765: f64, t48789: f64, t48813: f64, t931: f64, t13716: f64, t2932: f64, t10632: f64, t4471: f64, t47681: f64, t47686: f64, t47691: f64, t47695: f64, t47699: f64, t47703: f64, t47705: f64, t48085: f64, t48087: f64, t48090: f64, t48092: f64, t48096: f64, t41831: f64, t41833: f64, t47707: f64, t47709: f64, t47711: f64, t47713: f64, t47715: f64, t47717: f64, t47722: f64, t47724: f64, t47728: f64, t47730: f64, t47732: f64, t47736: f64, t47738: f64, t48098: f64, t48101: f64, t48103: f64, t41904: f64, t901: f64, t41863: f64, t41865: f64, t41870: f64, t41872: f64, t41874: f64, t41876: f64, t48112: f64, t48114: f64, t48116: f64, t48119: f64, t48122: f64, t48125: f64, t48128: f64, t48131: f64, t10588: f64, t4362: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t48833 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2379(t41656, t41658, t41660, t41662, t41675, t41678, t41680, t41682, t41684, t41713, t42245, t47744, t47748, t47761, t47765, t47769, t47777, t47781, t47785, t47787);
        let t48861 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2380(t10813, t4433, t10743, t10765, t10771, t10805, t10811, t14266, t14328, t14432, t14435, t14436, t14442, t1569, t2861, t2862, t2880, t2881, t2886, t2888, t2889, t311, t41984, t42154, t47791, t48747, t48750, t48765, t48789, t48813, t48833, t931);
        let (t48883, t48890, t48907) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2381(t13716, t2932, t10632, t4471, t47681, t47686, t47691, t47695, t47699, t47703, t47705, t48085, t48087, t48090, t48092);
        let t48920 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2382(t48096, t41831, t41833, t47707, t47709, t47711, t47713, t47715, t47717, t47722, t47724, t47728);
        let t48934 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2383(t47730, t41656, t41658, t41660, t47732, t47736, t47738, t47744, t47748, t48098, t48101, t48103);
        let t48960 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2384(t47705, t47707, t47730, t47681, t47686, t47691, t47695, t47699, t47703, t47709, t47711, t47713, t47715, t47717, t47722, t47724, t47728, t47732, t47736, t47738);
        let t48980 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2385(t41656, t41658, t41660, t41662, t41675, t41678, t41680, t41682, t41684, t41713, t41904, t47744, t47748, t47761, t47765, t47769, t47777, t47781, t47785, t47787);
        let (t48981, t48982, t48990) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2386(t48960, t48980, t901, t41662, t41675, t41678, t41682, t41684, t41863, t41865, t41870, t41872, t41874, t41876);
        let (t49004, t49009) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2387(t47761, t47765, t47769, t48112, t48114, t48116, t48119, t48122, t48125, t48128, t48131, t10588, t4362);
    (t48861, t48883, t48890, t48907, t48920, t48934, t48981, t48982, t48990, t49004, t49009)
}
