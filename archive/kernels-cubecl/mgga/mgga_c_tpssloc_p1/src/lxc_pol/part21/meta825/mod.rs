//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta825 (260520-c91 hierarchical CSE).
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
mod chunk9;
mod chunk10;
mod chunk11;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2900;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2901;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2902;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2903;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2904;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2905;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2906;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2907;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2908;
use chunk9::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2909;
use chunk10::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2910;
use chunk11::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2911;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta825<F: Float>(t42086: F, t42087: F, t59680: F, t59684: F, t59688: F, t59692: F, t59694: F, t60223: F, t60226: F, t60229: F, t60232: F, t60235: F, t60238: F, t60240: F, t59698: F, t60243: F, t60245: F, t60248: F, t60251: F, t60254: F, t60257: F, t60260: F, t60263: F, t60265: F, t60267: F, t60269: F, t60271: F, t60274: F, t60277: F, t47787: F, t59700: F, t59702: F, t59704: F, t59708: F, t59713: F, t59717: F, t59721: F, t59727: F, t59732: F, t59735: F, t59738: F, t59744: F, t60282: F, t60296: F, t59748: F, t59753: F, t59757: F, t59759: F, t59761: F, t59765: F, t59769: F, t60308: F, t60310: F, t60312: F, t60315: F, t60318: F, t60321: F, t60324: F, t60327: F, t60449: F, t60465: F, t60482: F, t60498: F, t893: F, t913: F, t41623: F, t5730: F, t41831: F, t47705: F, t47707: F, t47709: F, t47711: F, t47713: F, t47715: F, t47717: F, t47724: F, t47730: F, t47732: F, t48087: F, t48096: F, t48098: F, t41656: F, t41658: F, t41675: F, t41684: F, t41863: F, t41870: F, t41872: F, t47738: F, t48103: F, t48116: F, t59655: F, t60091: F, t60150: F, t60153: F, t60156: F, t48155: F, t48157: F, t48159: F, t48161: F, t48163: F, t48165: F, t48167: F, t59657: F, t60161: F, t60163: F, t60166: F, t60168: F, t60171: F, t60173: F, t60176: F, t59661: F, t59663: F, t59665: F, t59670: F, t59674: F, t59678: F, t60186: F, t60189: F, t60192: F, t60194: F, t60197: F, t60200: F, t60202: F, t60204: F, t60207: F, t42212: F, t42213: F) -> (F, F, F, F, F, F, F, F, F) {
        let t60513 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2900::<F>(t42086, t42087, t59680, t59684, t59688, t59692, t59694, t60223, t60226, t60229, t60232, t60235, t60238, t60240);
        let t60529 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2901::<F>(t59698, t60243, t60245, t60248, t60251, t60254, t60257, t60260, t60263, t60265, t60267, t60269, t60271, t60274, t60277);
        let t60546 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2902::<F>(t47787, t59700, t59702, t59704, t59708, t59713, t59717, t59721, t59727, t59732, t59735, t59738, t59744, t60282, t60296);
        let t60562 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2903::<F>(t59748, t59753, t59757, t59759, t59761, t59765, t59769, t60308, t60310, t60312, t60315, t60318, t60321, t60324, t60327);
        let (t60568, t60570) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2904::<F>(t60449, t60465, t60482, t60498, t60513, t60529, t60546, t60562, t893, t913, t41623, t5730);
        let t60585 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2905::<F>(t41831, t47705, t47707, t47709, t47711, t47713, t47715, t47717, t47724, t47730, t47732, t48087, t48096, t48098);
        let t60601 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2906::<F>(t41656, t41658, t41675, t41684, t41863, t41870, t41872, t47738, t48103, t48116, t59655, t60091, t60150, t60153, t60156);
        let t60618 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2907::<F>(t48155, t48157, t48159, t48161, t48163, t48165, t48167, t59657, t60161, t60163, t60166, t60168, t60171, t60173, t60176);
        let t60634 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2908::<F>(t59661, t59663, t59665, t59670, t59674, t59678, t60186, t60189, t60192, t60194, t60197, t60200, t60202, t60204, t60207);
        let t60649 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2909::<F>(t42212, t42213, t59680, t59684, t59688, t59692, t59694, t60223, t60226, t60229, t60232, t60235, t60238, t60240);
        let t60665 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2910::<F>(t59698, t60243, t60245, t60248, t60251, t60254, t60257, t60260, t60263, t60265, t60267, t60269, t60271, t60274, t60277);
        let t60682 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2911::<F>(t47787, t59700, t59702, t59704, t59708, t59713, t59717, t59721, t59727, t59732, t59735, t59738, t59744, t60282, t60296);
    (t60568, t60570, t60585, t60601, t60618, t60634, t60649, t60665, t60682)
}
