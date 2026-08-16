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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta825(t42086: f64, t42087: f64, t59680: f64, t59684: f64, t59688: f64, t59692: f64, t59694: f64, t60223: f64, t60226: f64, t60229: f64, t60232: f64, t60235: f64, t60238: f64, t60240: f64, t59698: f64, t60243: f64, t60245: f64, t60248: f64, t60251: f64, t60254: f64, t60257: f64, t60260: f64, t60263: f64, t60265: f64, t60267: f64, t60269: f64, t60271: f64, t60274: f64, t60277: f64, t47787: f64, t59700: f64, t59702: f64, t59704: f64, t59708: f64, t59713: f64, t59717: f64, t59721: f64, t59727: f64, t59732: f64, t59735: f64, t59738: f64, t59744: f64, t60282: f64, t60296: f64, t59748: f64, t59753: f64, t59757: f64, t59759: f64, t59761: f64, t59765: f64, t59769: f64, t60308: f64, t60310: f64, t60312: f64, t60315: f64, t60318: f64, t60321: f64, t60324: f64, t60327: f64, t60449: f64, t60465: f64, t60482: f64, t60498: f64, t893: f64, t913: f64, t41623: f64, t5730: f64, t41831: f64, t47705: f64, t47707: f64, t47709: f64, t47711: f64, t47713: f64, t47715: f64, t47717: f64, t47724: f64, t47730: f64, t47732: f64, t48087: f64, t48096: f64, t48098: f64, t41656: f64, t41658: f64, t41675: f64, t41684: f64, t41863: f64, t41870: f64, t41872: f64, t47738: f64, t48103: f64, t48116: f64, t59655: f64, t60091: f64, t60150: f64, t60153: f64, t60156: f64, t48155: f64, t48157: f64, t48159: f64, t48161: f64, t48163: f64, t48165: f64, t48167: f64, t59657: f64, t60161: f64, t60163: f64, t60166: f64, t60168: f64, t60171: f64, t60173: f64, t60176: f64, t59661: f64, t59663: f64, t59665: f64, t59670: f64, t59674: f64, t59678: f64, t60186: f64, t60189: f64, t60192: f64, t60194: f64, t60197: f64, t60200: f64, t60202: f64, t60204: f64, t60207: f64, t42212: f64, t42213: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t60513 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2900(t42086, t42087, t59680, t59684, t59688, t59692, t59694, t60223, t60226, t60229, t60232, t60235, t60238, t60240);
        let t60529 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2901(t59698, t60243, t60245, t60248, t60251, t60254, t60257, t60260, t60263, t60265, t60267, t60269, t60271, t60274, t60277);
        let t60546 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2902(t47787, t59700, t59702, t59704, t59708, t59713, t59717, t59721, t59727, t59732, t59735, t59738, t59744, t60282, t60296);
        let t60562 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2903(t59748, t59753, t59757, t59759, t59761, t59765, t59769, t60308, t60310, t60312, t60315, t60318, t60321, t60324, t60327);
        let (t60568, t60570) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2904(t60449, t60465, t60482, t60498, t60513, t60529, t60546, t60562, t893, t913, t41623, t5730);
        let t60585 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2905(t41831, t47705, t47707, t47709, t47711, t47713, t47715, t47717, t47724, t47730, t47732, t48087, t48096, t48098);
        let t60601 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2906(t41656, t41658, t41675, t41684, t41863, t41870, t41872, t47738, t48103, t48116, t59655, t60091, t60150, t60153, t60156);
        let t60618 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2907(t48155, t48157, t48159, t48161, t48163, t48165, t48167, t59657, t60161, t60163, t60166, t60168, t60171, t60173, t60176);
        let t60634 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2908(t59661, t59663, t59665, t59670, t59674, t59678, t60186, t60189, t60192, t60194, t60197, t60200, t60202, t60204, t60207);
        let t60649 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2909(t42212, t42213, t59680, t59684, t59688, t59692, t59694, t60223, t60226, t60229, t60232, t60235, t60238, t60240);
        let t60665 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2910(t59698, t60243, t60245, t60248, t60251, t60254, t60257, t60260, t60263, t60265, t60267, t60269, t60271, t60274, t60277);
        let t60682 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2911(t47787, t59700, t59702, t59704, t59708, t59713, t59717, t59721, t59727, t59732, t59735, t59738, t59744, t60282, t60296);
    (t60568, t60570, t60585, t60601, t60618, t60634, t60649, t60665, t60682)
}
