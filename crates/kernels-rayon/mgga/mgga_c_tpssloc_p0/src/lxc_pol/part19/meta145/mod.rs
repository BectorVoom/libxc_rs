//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta145 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk745;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk746;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk747;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk748;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk749;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk750;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta145(t1352: f64, t3901: f64, t1380: f64, t3851: f64, t3856: f64, t3879: f64, t553: f64, t1332: f64, t1336: f64, t1381: f64, t1383: f64, t3773: f64, t3777: f64, t3898: f64, t544: f64, t564: f64, t1378: f64, t1375: f64, t1386: f64, t3753: f64, t3755: f64, t3758: f64, t3880: f64, t3882: f64, t3889: f64, t568: f64, t193: f64, t532: f64, t1388: f64, t1390: f64, t1297: f64, t1307: f64, t2408: f64, t2417: f64, t3683: f64, t3686: f64, t3688: f64, t3690: f64, t3693: f64, t3695: f64, t3697: f64, t3698: f64, t3701: f64, t3719: f64, t3813: f64, t533: f64, t531: f64, t571: f64, t2423: f64, t2426: f64, t2486: f64, t3734: f64, t3816: f64, t3819: f64, t3821: f64, t3823: f64, t3825: f64, t3828: f64, t3830: f64, t3832: f64, t3834: f64, t3836: f64, t113: f64, t1266: f64, t1271: f64, t1393: f64, t2312: f64, t2314: f64, t2320: f64, t2323: f64, t2364: f64, t3652: f64, t3660: f64, t510: f64, t513: f64, t574: f64, t650: f64, t652: f64, t672: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3902, t3905, t3907, t3909, t3911) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk745(t1352, t3901, t1380, t3851, t3856, t3879, t553, t1332, t1336, t1381, t1383, t3773, t3777, t3898, t544, t564);
        let (t3912, t3914) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk746(t1378, t3911, t1375, t1386, t3753, t3755, t3758, t3880, t3882, t3889, t568);
        let t3918 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk747(t193, t532);
        let (t3919, t3923) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk748(t1388, t1390, t1297, t1307, t193, t2408, t2417, t3683, t3686, t3688, t3690, t3693, t3695, t3697, t3698, t3701, t3719, t3813, t3914, t3918, t533);
        let (t3924, t3928) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk749(t531, t571, t193, t2423, t2426, t2486, t3734, t3816, t3819, t3821, t3823, t3825, t3828, t3830, t3832, t3834, t3836);
        let (t3929, t3931) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk750(t3923, t3928, t113, t1266, t1271, t1393, t2312, t2314, t2320, t2323, t2364, t3652, t3660, t510, t513, t574, t650, t652, t672);
    (t3902, t3905, t3907, t3909, t3911, t3912, t3914, t3918, t3919, t3924, t3929, t3931)
}
