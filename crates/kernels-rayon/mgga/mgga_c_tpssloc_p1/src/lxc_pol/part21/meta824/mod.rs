//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta824 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2895;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2896;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2897;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2898;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2899;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta824(t2793: f64, t2842: f64, t5727: f64, t4395: f64, t2792: f64, t913: f64, t10650: f64, t14332: f64, t14436: f64, t14450: f64, t1581: f64, t2886: f64, t2888: f64, t4472: f64, t48776: f64, t48783: f64, t48854: f64, t49404: f64, t49478: f64, t60354: f64, t60359: f64, t60360: f64, t60371: f64, t60374: f64, t60377: f64, t60381: f64, t60384: f64, t60387: f64, t60391: f64, t931: f64, t2885: f64, t5737: f64, t2904: f64, t5769: f64, t2844: f64, t17423: f64, t2787: f64, t41831: f64, t47705: f64, t47707: f64, t47709: f64, t47711: f64, t47713: f64, t47715: f64, t47717: f64, t47724: f64, t47730: f64, t47732: f64, t48087: f64, t48096: f64, t48098: f64, t41656: f64, t41658: f64, t41675: f64, t41684: f64, t41863: f64, t41870: f64, t41872: f64, t47738: f64, t48103: f64, t48116: f64, t59655: f64, t60091: f64, t60150: f64, t60153: f64, t60156: f64, t48155: f64, t48157: f64, t48159: f64, t48161: f64, t48163: f64, t48165: f64, t48167: f64, t59657: f64, t60161: f64, t60163: f64, t60166: f64, t60168: f64, t60171: f64, t60173: f64, t60176: f64, t59661: f64, t59663: f64, t59665: f64, t59670: f64, t59674: f64, t59678: f64, t60186: f64, t60189: f64, t60192: f64, t60194: f64, t60197: f64, t60200: f64, t60202: f64, t60204: f64, t60207: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t60394, t60395, t60398, t60400, t60401) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2895(t2793, t2842, t5727, t4395, t2792, t913, t10650, t14332, t14436, t14450, t1581, t2886, t2888, t4472, t48776, t48783, t48854, t49404, t49478, t60354, t60359, t60360, t60371, t60374, t60377, t60381, t60384, t60387, t60391, t931);
        let (t60407, t60424, t60429, t60434, t60449) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2896(t2885, t5737, t2904, t5769, t2842, t2844, t60395, t17423, t2787, t41831, t47705, t47707, t47709, t47711, t47713, t47715, t47717, t47724, t47730, t47732, t48087, t48096, t48098);
        let t60465 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2897(t41656, t41658, t41675, t41684, t41863, t41870, t41872, t47738, t48103, t48116, t59655, t60091, t60150, t60153, t60156);
        let t60482 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2898(t48155, t48157, t48159, t48161, t48163, t48165, t48167, t59657, t60161, t60163, t60166, t60168, t60171, t60173, t60176);
        let t60498 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2899(t59661, t59663, t59665, t59670, t59674, t59678, t60186, t60189, t60192, t60194, t60197, t60200, t60202, t60204, t60207);
    (t60394, t60398, t60400, t60401, t60407, t60424, t60429, t60434, t60449, t60465, t60482, t60498)
}
