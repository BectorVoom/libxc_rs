//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta145 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk745;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk746;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk747;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk748;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk749;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk750;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta145<F: Float>(t1352: F, t3901: F, t1380: F, t3851: F, t3856: F, t3879: F, t553: F, t1332: F, t1336: F, t1381: F, t1383: F, t3773: F, t3777: F, t3898: F, t544: F, t564: F, t1378: F, t1375: F, t1386: F, t3753: F, t3755: F, t3758: F, t3880: F, t3882: F, t3889: F, t568: F, t193: F, t532: F, t1388: F, t1390: F, t1297: F, t1307: F, t2408: F, t2417: F, t3683: F, t3686: F, t3688: F, t3690: F, t3693: F, t3695: F, t3697: F, t3698: F, t3701: F, t3719: F, t3813: F, t533: F, t531: F, t571: F, t2423: F, t2426: F, t2486: F, t3734: F, t3816: F, t3819: F, t3821: F, t3823: F, t3825: F, t3828: F, t3830: F, t3832: F, t3834: F, t3836: F, t113: F, t1266: F, t1271: F, t1393: F, t2312: F, t2314: F, t2320: F, t2323: F, t2364: F, t3652: F, t3660: F, t510: F, t513: F, t574: F, t650: F, t652: F, t672: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3902, t3905, t3907, t3909, t3911) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk745::<F>(t1352, t3901, t1380, t3851, t3856, t3879, t553, t1332, t1336, t1381, t1383, t3773, t3777, t3898, t544, t564);
        let (t3912, t3914) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk746::<F>(t1378, t3911, t1375, t1386, t3753, t3755, t3758, t3880, t3882, t3889, t568);
        let t3918 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk747::<F>(t193, t532);
        let (t3919, t3923) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk748::<F>(t1388, t1390, t1297, t1307, t193, t2408, t2417, t3683, t3686, t3688, t3690, t3693, t3695, t3697, t3698, t3701, t3719, t3813, t3914, t3918, t533);
        let (t3924, t3928) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk749::<F>(t531, t571, t193, t2423, t2426, t2486, t3734, t3816, t3819, t3821, t3823, t3825, t3828, t3830, t3832, t3834, t3836);
        let (t3929, t3931) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk750::<F>(t3923, t3928, t113, t1266, t1271, t1393, t2312, t2314, t2320, t2323, t2364, t3652, t3660, t510, t513, t574, t650, t652, t672);
    (t3902, t3905, t3907, t3909, t3911, t3912, t3914, t3918, t3919, t3924, t3929, t3931)
}
