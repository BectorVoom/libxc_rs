//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1055 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3730;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3731;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3732;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3733;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3734;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3735;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3736;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1055(t11249: f64, t1248: f64, t1284: f64, t20849: f64, t3624: f64, t12772: f64, t17729: f64, t21036: f64, t3625: f64, t44250: f64, t6639: f64, t17423: f64, t21049: f64, t21439: f64, t21082: f64, t1250: f64, t12832: f64, t13053: f64, t17391: f64, t17396: f64, t17401: f64, t17602: f64, t17656: f64, t17724: f64, t21300: f64, t3629: f64, t3631: f64, t3718: f64, t3720: f64, t3723: f64, t5348: f64, t56879: f64, t57040: f64, t57569: f64, t59066: f64, t69839: f64, t21160: f64, t12784: f64, t12910: f64, t13312: f64, t17459: f64, t17461: f64, t17644: f64, t20747: f64, t20795: f64, t21040: f64, t21228: f64, t21298: f64, t3626: f64, t3628: f64, t44459: f64, t44466: f64, t44609: f64, t5340: f64, t5351: f64, t5405: f64, t57147: f64, t57584: f64, t57586: f64, t57590: f64, t57602: f64, t6622: f64, t1214: f64, t12866: f64, t1715: f64, t17353: f64, t17515: f64, t17623: f64, t17709: f64, t17711: f64, t17747: f64, t17748: f64, t20933: f64, t20934: f64, t3584: f64, t44561: f64, t44607: f64, t44952: f64, t5056: f64, t56981: f64, t57604: f64, t57615: f64, t57635: f64, t57660: f64, t57687: f64, t19666: f64, t20926: f64, t15904: f64, t17394: f64, t13127: f64, t1469: f64, t606: f64, t3682: f64, t6667: f64, t20900: f64, t73: f64, t17654: f64, t17693: f64, t17694: f64, t17695: f64, t17756: f64, t17794: f64, t20767: f64, t20932: f64, t21063: f64, t3362: f64, t3367: f64, t3663: f64, t372: f64, t44230: f64, t44458: f64, t44517: f64, t5277: f64, t5333: f64, t5352: f64, t56977: f64, t57689: f64, t58909: f64, t6640: f64, t12987: f64, t5390: f64, t17347: f64, t17448: f64, t17635: f64, t17753: f64, t17754: f64, t5402: f64, t57100: f64, t57726: f64, t57735: f64, t57743: f64, t57746: f64, t57749: f64, t57770: f64, t57773: f64, t57780: f64, t6688: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t70794, t70800, t70806, t70809, t70811) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3730(t11249, t1248, t1284, t20849, t3624, t12772, t17729, t21036, t3625, t44250, t6639, t17423, t21049);
        let (t70824, t70830) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3731(t21439, t3624, t1248, t21082, t1250, t12832, t13053, t17391, t17396, t17401, t17602, t17656, t17724, t21300, t3629, t3631, t3718, t3720, t3723, t5348, t56879, t57040, t57569, t59066, t69839, t70794, t70800, t70806, t70809, t70811);
        let t70872 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3732(t12772, t21160, t3625, t12784, t12910, t13312, t17459, t17461, t17644, t20747, t20795, t21040, t21228, t21298, t3626, t3628, t3720, t44459, t44466, t44609, t5340, t5351, t5405, t57147, t57584, t57586, t57590, t57602);
        let (t70890, t70907) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3733(t11249, t6622, t1214, t1250, t12866, t1715, t17353, t17515, t17623, t17709, t17711, t17747, t17748, t20795, t20933, t20934, t3584, t3720, t44561, t44607, t44952, t5056, t56981, t57604, t57615, t57635, t57660, t57687);
        let (t70910, t70914, t70916, t70917, t70932, t70933, t70942) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3734(t19666, t5405, t12832, t20926, t15904, t17394, t13127, t1248, t1469, t606, t3682, t6667);
        let (t70944, t70953) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3735(t20900, t73, t12866, t17654, t17693, t17694, t17695, t17756, t17794, t20767, t20932, t21063, t3362, t3367, t3663, t3718, t372, t3720, t44230, t44458, t44517, t5277, t5333, t5352, t56977, t57689, t58909, t6640, t70910, t70914, t70917, t70933, t70942);
        let t70978 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3736(t12987, t5390, t1250, t12910, t17347, t17448, t17635, t17753, t17754, t3584, t3720, t5402, t57100, t57726, t57735, t57743, t57746, t57749, t57770, t57773, t57780, t6688, t70890);
    (t70824, t70830, t70872, t70890, t70907, t70910, t70916, t70932, t70933, t70944, t70953, t70978)
}
