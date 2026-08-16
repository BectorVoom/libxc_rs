//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta824 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2895;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2896;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2897;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2898;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2899;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta824<F: Float>(t2793: F, t2842: F, t5727: F, t4395: F, t2792: F, t913: F, t10650: F, t14332: F, t14436: F, t14450: F, t1581: F, t2886: F, t2888: F, t4472: F, t48776: F, t48783: F, t48854: F, t49404: F, t49478: F, t60354: F, t60359: F, t60360: F, t60371: F, t60374: F, t60377: F, t60381: F, t60384: F, t60387: F, t60391: F, t931: F, t2885: F, t5737: F, t2904: F, t5769: F, t2844: F, t17423: F, t2787: F, t41831: F, t47705: F, t47707: F, t47709: F, t47711: F, t47713: F, t47715: F, t47717: F, t47724: F, t47730: F, t47732: F, t48087: F, t48096: F, t48098: F, t41656: F, t41658: F, t41675: F, t41684: F, t41863: F, t41870: F, t41872: F, t47738: F, t48103: F, t48116: F, t59655: F, t60091: F, t60150: F, t60153: F, t60156: F, t48155: F, t48157: F, t48159: F, t48161: F, t48163: F, t48165: F, t48167: F, t59657: F, t60161: F, t60163: F, t60166: F, t60168: F, t60171: F, t60173: F, t60176: F, t59661: F, t59663: F, t59665: F, t59670: F, t59674: F, t59678: F, t60186: F, t60189: F, t60192: F, t60194: F, t60197: F, t60200: F, t60202: F, t60204: F, t60207: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t60394, t60395, t60398, t60400, t60401) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2895::<F>(t2793, t2842, t5727, t4395, t2792, t913, t10650, t14332, t14436, t14450, t1581, t2886, t2888, t4472, t48776, t48783, t48854, t49404, t49478, t60354, t60359, t60360, t60371, t60374, t60377, t60381, t60384, t60387, t60391, t931);
        let (t60407, t60424, t60429, t60434, t60449) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2896::<F>(t2885, t5737, t2904, t5769, t2842, t2844, t60395, t17423, t2787, t41831, t47705, t47707, t47709, t47711, t47713, t47715, t47717, t47724, t47730, t47732, t48087, t48096, t48098);
        let t60465 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2897::<F>(t41656, t41658, t41675, t41684, t41863, t41870, t41872, t47738, t48103, t48116, t59655, t60091, t60150, t60153, t60156);
        let t60482 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2898::<F>(t48155, t48157, t48159, t48161, t48163, t48165, t48167, t59657, t60161, t60163, t60166, t60168, t60171, t60173, t60176);
        let t60498 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2899::<F>(t59661, t59663, t59665, t59670, t59674, t59678, t60186, t60189, t60192, t60194, t60197, t60200, t60202, t60204, t60207);
    (t60394, t60398, t60400, t60401, t60407, t60424, t60429, t60434, t60449, t60465, t60482, t60498)
}
