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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta727(t5: f64, t104758: f64, t104783: f64, t104813: f64, t104858: f64, t104885: f64, t104916: f64, t104942: f64, t104971: f64, t112: f64, t671: f64, t7982: f64, t111: f64, t29485: f64, t104729: f64, t1458: f64, t19534: f64, t24932: f64, t27863: f64, t27888: f64, t33690: f64, t4072: f64, t5493: f64, t7266: f64, t96238: f64, t96659: f64, t96661: f64, t96663: f64, t96665: f64, t96667: f64, t96669: f64, t96671: f64, t96673: f64, t96675: f64, t96677: f64, t96679: f64, t96681: f64, t96685: f64, t96704: f64, t96706: f64, t96708: f64, t96711: f64, t96731: f64, t1266: f64, t12725: f64, t19456: f64, t27879: f64, t29486: f64, t4028: f64, t574: f64, t7989: f64, t96784: f64, t96786: f64, t96789: f64, t96792: f64, t96796: f64, t96799: f64, t96802: f64, t96805: f64, t96807: f64, t96813: f64, t96815: f64, t96818: f64, t96827: f64, t96829: f64, t1459: f64, t1774: f64, t19461: f64, t2165: f64, t27290: f64, t27293: f64, t27371: f64, t5457: f64, t652: f64, t672: f64, t7408: f64, t7458: f64, t96833: f64, t96837: f64, t96839: f64, t96842: f64, t96844: f64, t96846: f64, t97777: f64, t97779: f64, t97783: f64, t97785: f64, t97788: f64, t1442: f64, t19451: f64, t20109: f64, t27858: f64, t29848: f64, t4037: f64, t4073: f64, t5460: f64, t650: f64, t7271: f64, t8103: f64, t97792: f64, t97794: f64, t97796: f64, t97798: f64, t97800: f64, t97802: f64, t97805: f64, t97808: f64, t97811: f64, t20127: f64, t2314: f64, t29501: f64, t4034: f64, t4077: f64, t97820: f64, t97829: f64, t97831: f64, t97833: f64, t97835: f64, t97836: f64, t97839: f64, t97842: f64, t97844: f64, t97846: f64, t97848: f64, t97850: f64, t97854: f64, t510: f64, t5361: f64, t8107: f64, t97856: f64, t97858: f64, t97862: f64, t97865: f64, t97869: f64, t97871: f64, t97874: f64, t97878: f64, t97880: f64, t97887: f64, t97889: f64, t97892: f64, t97893: f64, t97897: f64, t20100: f64, t20136: f64, t20143: f64, t29855: f64, t5450: f64, t5494: f64, t6287: f64, t6468: f64, t7264: f64, t7412: f64, t97899: f64, t97905: f64, t97910: f64, t97914: f64, t97916: f64, t97919: f64, t97923: f64, t97925: f64, t97928: f64, t100828: f64, t100833: f64, t100835: f64, t100838: f64, t100840: f64, t100854: f64, t100861: f64, t100863: f64, t29493: f64, t4026: f64, t5107: f64, t7983: f64, t97930: f64, t97932: f64, t97935: f64, t97937: f64, t97941: f64, t97942: f64, t97947: f64, t97949: f64, t104727: f64, t29865: f64, t2169: f64, t100871: f64, t100873: f64, t100875: f64, t100879: f64, t100883: f64, t100885: f64, t100887: f64, t100890: f64, t100893: f64, t100897: f64, t100899: f64, t100902: f64, t24969: f64, t5456: f64, t577: f64, t7423: f64) -> (f64, f64) {
        let (t104976, t104977, t104990) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2353(t5, t104758, t104783, t104813, t104858, t104885, t104916, t104942, t104971, t112, t671, t7982, t111, t29485);
        let t104995 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2354(t104729, t104976, t104977, t104990, t1458, t19534, t24932, t27863, t27888, t33690, t4072, t5493, t671, t7266, t96238, t96659, t96661, t96663, t96665);
        let t104996 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2355(t96667, t96669, t96671, t96673, t96675, t96677, t96679, t96681, t96685, t96704, t96706, t96708, t96711, t96731);
        let t105005 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2356(t104995, t104996, t1266, t12725, t19456, t27879, t29486, t4028, t574, t7989, t96784, t96786, t96789, t96792, t96796, t96799, t96802, t96805, t96807, t96813, t96815, t96818, t96827, t96829);
        let t105024 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2357(t104990, t1459, t1774, t19461, t19534, t2165, t27290, t27293, t27371, t4028, t5457, t652, t672, t7408, t7458, t96238, t96833, t96837, t96839, t96842, t96844, t96846, t97777, t97779, t97783, t97785, t97788);
        let t105045 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2358(t104977, t1442, t1459, t19451, t20109, t24932, t27858, t27863, t27888, t29848, t4037, t4072, t4073, t5460, t650, t652, t7266, t7271, t8103, t97792, t97794, t97796, t97798, t97800, t97802, t97805, t97808, t97811);
        let t105062 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2359(t1458, t20127, t2314, t27858, t27863, t29501, t29848, t4034, t4077, t652, t671, t7266, t97820, t97829, t97831, t97833, t97835, t97836, t97839, t97842, t97844, t97846, t97848, t97850, t97854);
        let t105073 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2360(t104729, t104976, t27290, t4028, t510, t5361, t5493, t652, t7408, t8107, t97856, t97858, t97862, t97865, t97869, t97871, t97874, t97878, t97880, t97887, t97889, t97892, t97893, t97897);
        let t105092 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2361(t20100, t20136, t20143, t2314, t24932, t27888, t29855, t4034, t5450, t5494, t6287, t6468, t7264, t7266, t7408, t7412, t97899, t97905, t97910, t97914, t97916, t97919, t97923, t97925, t97928);
        let t105099 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2362(t100828, t100833, t100835, t100838, t100840, t100854, t100861, t100863, t1266, t29493, t4026, t5107, t7983, t8103, t97930, t97932, t97935, t97937, t97941, t97942, t97947, t97949);
        let (t105102, t105105, t105108) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2363(t104727, t105005, t105024, t105045, t105062, t105073, t105092, t105099, t112, t29865, t2169, t671);
        let t105115 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2364(t100871, t100873, t100875, t100879, t100883, t100885, t100887, t100890, t100893, t100897, t100899, t100902, t105102, t105105, t105108, t19534, t24969, t5456, t5493, t577, t671, t7423);
    (t105102, t105115)
}
