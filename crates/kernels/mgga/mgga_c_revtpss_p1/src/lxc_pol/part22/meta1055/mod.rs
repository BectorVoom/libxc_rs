//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1055 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3730;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3731;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3732;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3733;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3734;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3735;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3736;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1055<F: Float>(t11249: F, t1248: F, t1284: F, t20849: F, t3624: F, t12772: F, t17729: F, t21036: F, t3625: F, t44250: F, t6639: F, t17423: F, t21049: F, t21439: F, t21082: F, t1250: F, t12832: F, t13053: F, t17391: F, t17396: F, t17401: F, t17602: F, t17656: F, t17724: F, t21300: F, t3629: F, t3631: F, t3718: F, t3720: F, t3723: F, t5348: F, t56879: F, t57040: F, t57569: F, t59066: F, t69839: F, t21160: F, t12784: F, t12910: F, t13312: F, t17459: F, t17461: F, t17644: F, t20747: F, t20795: F, t21040: F, t21228: F, t21298: F, t3626: F, t3628: F, t44459: F, t44466: F, t44609: F, t5340: F, t5351: F, t5405: F, t57147: F, t57584: F, t57586: F, t57590: F, t57602: F, t6622: F, t1214: F, t12866: F, t1715: F, t17353: F, t17515: F, t17623: F, t17709: F, t17711: F, t17747: F, t17748: F, t20933: F, t20934: F, t3584: F, t44561: F, t44607: F, t44952: F, t5056: F, t56981: F, t57604: F, t57615: F, t57635: F, t57660: F, t57687: F, t19666: F, t20926: F, t15904: F, t17394: F, t13127: F, t1469: F, t606: F, t3682: F, t6667: F, t20900: F, t73: F, t17654: F, t17693: F, t17694: F, t17695: F, t17756: F, t17794: F, t20767: F, t20932: F, t21063: F, t3362: F, t3367: F, t3663: F, t372: F, t44230: F, t44458: F, t44517: F, t5277: F, t5333: F, t5352: F, t56977: F, t57689: F, t58909: F, t6640: F, t12987: F, t5390: F, t17347: F, t17448: F, t17635: F, t17753: F, t17754: F, t5402: F, t57100: F, t57726: F, t57735: F, t57743: F, t57746: F, t57749: F, t57770: F, t57773: F, t57780: F, t6688: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t70794, t70800, t70806, t70809, t70811) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3730::<F>(t11249, t1248, t1284, t20849, t3624, t12772, t17729, t21036, t3625, t44250, t6639, t17423, t21049);
        let (t70824, t70830) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3731::<F>(t21439, t3624, t1248, t21082, t1250, t12832, t13053, t17391, t17396, t17401, t17602, t17656, t17724, t21300, t3629, t3631, t3718, t3720, t3723, t5348, t56879, t57040, t57569, t59066, t69839, t70794, t70800, t70806, t70809, t70811);
        let t70872 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3732::<F>(t12772, t21160, t3625, t12784, t12910, t13312, t17459, t17461, t17644, t20747, t20795, t21040, t21228, t21298, t3626, t3628, t3720, t44459, t44466, t44609, t5340, t5351, t5405, t57147, t57584, t57586, t57590, t57602);
        let (t70890, t70907) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3733::<F>(t11249, t6622, t1214, t1250, t12866, t1715, t17353, t17515, t17623, t17709, t17711, t17747, t17748, t20795, t20933, t20934, t3584, t3720, t44561, t44607, t44952, t5056, t56981, t57604, t57615, t57635, t57660, t57687);
        let (t70910, t70914, t70916, t70917, t70932, t70933, t70942) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3734::<F>(t19666, t5405, t12832, t20926, t15904, t17394, t13127, t1248, t1469, t606, t3682, t6667);
        let (t70944, t70953) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3735::<F>(t20900, t73, t12866, t17654, t17693, t17694, t17695, t17756, t17794, t20767, t20932, t21063, t3362, t3367, t3663, t3718, t372, t3720, t44230, t44458, t44517, t5277, t5333, t5352, t56977, t57689, t58909, t6640, t70910, t70914, t70917, t70933, t70942);
        let t70978 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3736::<F>(t12987, t5390, t1250, t12910, t17347, t17448, t17635, t17753, t17754, t3584, t3720, t5402, t57100, t57726, t57735, t57743, t57746, t57749, t57770, t57773, t57780, t6688, t70890);
    (t70824, t70830, t70872, t70890, t70907, t70910, t70916, t70932, t70933, t70944, t70953, t70978)
}
