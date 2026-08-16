//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta727 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2353;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2354;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2355;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2356;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2357;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2358;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2359;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2360;
use chunk8::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2361;
use chunk9::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2362;
use chunk10::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2363;
use chunk11::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2364;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta727<F: Float>(t5: F, t104758: F, t104783: F, t104813: F, t104858: F, t104885: F, t104916: F, t104942: F, t104971: F, t112: F, t671: F, t7982: F, t111: F, t29485: F, t104729: F, t1458: F, t19534: F, t24932: F, t27863: F, t27888: F, t33690: F, t4072: F, t5493: F, t7266: F, t96238: F, t96659: F, t96661: F, t96663: F, t96665: F, t96667: F, t96669: F, t96671: F, t96673: F, t96675: F, t96677: F, t96679: F, t96681: F, t96685: F, t96704: F, t96706: F, t96708: F, t96711: F, t96731: F, t1266: F, t12725: F, t19456: F, t27879: F, t29486: F, t4028: F, t574: F, t7989: F, t96784: F, t96786: F, t96789: F, t96792: F, t96796: F, t96799: F, t96802: F, t96805: F, t96807: F, t96813: F, t96815: F, t96818: F, t96827: F, t96829: F, t1459: F, t1774: F, t19461: F, t2165: F, t27290: F, t27293: F, t27371: F, t5457: F, t652: F, t672: F, t7408: F, t7458: F, t96833: F, t96837: F, t96839: F, t96842: F, t96844: F, t96846: F, t97777: F, t97779: F, t97783: F, t97785: F, t97788: F, t1442: F, t19451: F, t20109: F, t27858: F, t29848: F, t4037: F, t4073: F, t5460: F, t650: F, t7271: F, t8103: F, t97792: F, t97794: F, t97796: F, t97798: F, t97800: F, t97802: F, t97805: F, t97808: F, t97811: F, t20127: F, t2314: F, t29501: F, t4034: F, t4077: F, t97820: F, t97829: F, t97831: F, t97833: F, t97835: F, t97836: F, t97839: F, t97842: F, t97844: F, t97846: F, t97848: F, t97850: F, t97854: F, t510: F, t5361: F, t8107: F, t97856: F, t97858: F, t97862: F, t97865: F, t97869: F, t97871: F, t97874: F, t97878: F, t97880: F, t97887: F, t97889: F, t97892: F, t97893: F, t97897: F, t20100: F, t20136: F, t20143: F, t29855: F, t5450: F, t5494: F, t6287: F, t6468: F, t7264: F, t7412: F, t97899: F, t97905: F, t97910: F, t97914: F, t97916: F, t97919: F, t97923: F, t97925: F, t97928: F, t100828: F, t100833: F, t100835: F, t100838: F, t100840: F, t100854: F, t100861: F, t100863: F, t29493: F, t4026: F, t5107: F, t7983: F, t97930: F, t97932: F, t97935: F, t97937: F, t97941: F, t97942: F, t97947: F, t97949: F, t104727: F, t29865: F, t2169: F, t100871: F, t100873: F, t100875: F, t100879: F, t100883: F, t100885: F, t100887: F, t100890: F, t100893: F, t100897: F, t100899: F, t100902: F, t24969: F, t5456: F, t577: F, t7423: F) -> (F, F) {
        let (t104976, t104977, t104990) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2353::<F>(t5, t104758, t104783, t104813, t104858, t104885, t104916, t104942, t104971, t112, t671, t7982, t111, t29485);
        let t104995 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2354::<F>(t104729, t104976, t104977, t104990, t1458, t19534, t24932, t27863, t27888, t33690, t4072, t5493, t671, t7266, t96238, t96659, t96661, t96663, t96665);
        let t104996 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2355::<F>(t96667, t96669, t96671, t96673, t96675, t96677, t96679, t96681, t96685, t96704, t96706, t96708, t96711, t96731);
        let t105005 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2356::<F>(t104995, t104996, t1266, t12725, t19456, t27879, t29486, t4028, t574, t7989, t96784, t96786, t96789, t96792, t96796, t96799, t96802, t96805, t96807, t96813, t96815, t96818, t96827, t96829);
        let t105024 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2357::<F>(t104990, t1459, t1774, t19461, t19534, t2165, t27290, t27293, t27371, t4028, t5457, t652, t672, t7408, t7458, t96238, t96833, t96837, t96839, t96842, t96844, t96846, t97777, t97779, t97783, t97785, t97788);
        let t105045 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2358::<F>(t104977, t1442, t1459, t19451, t20109, t24932, t27858, t27863, t27888, t29848, t4037, t4072, t4073, t5460, t650, t652, t7266, t7271, t8103, t97792, t97794, t97796, t97798, t97800, t97802, t97805, t97808, t97811);
        let t105062 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2359::<F>(t1458, t20127, t2314, t27858, t27863, t29501, t29848, t4034, t4077, t652, t671, t7266, t97820, t97829, t97831, t97833, t97835, t97836, t97839, t97842, t97844, t97846, t97848, t97850, t97854);
        let t105073 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2360::<F>(t104729, t104976, t27290, t4028, t510, t5361, t5493, t652, t7408, t8107, t97856, t97858, t97862, t97865, t97869, t97871, t97874, t97878, t97880, t97887, t97889, t97892, t97893, t97897);
        let t105092 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2361::<F>(t20100, t20136, t20143, t2314, t24932, t27888, t29855, t4034, t5450, t5494, t6287, t6468, t7264, t7266, t7408, t7412, t97899, t97905, t97910, t97914, t97916, t97919, t97923, t97925, t97928);
        let t105099 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2362::<F>(t100828, t100833, t100835, t100838, t100840, t100854, t100861, t100863, t1266, t29493, t4026, t5107, t7983, t8103, t97930, t97932, t97935, t97937, t97941, t97942, t97947, t97949);
        let (t105102, t105105, t105108) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2363::<F>(t104727, t105005, t105024, t105045, t105062, t105073, t105092, t105099, t112, t29865, t2169, t671);
        let t105115 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2364::<F>(t100871, t100873, t100875, t100879, t100883, t100885, t100887, t100890, t100893, t100897, t100899, t100902, t105102, t105105, t105108, t19534, t24969, t5456, t5493, t577, t671, t7423);
    (t105102, t105115)
}
