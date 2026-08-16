//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta475 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1420;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1421;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1422;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1423;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1424;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1425;
use chunk6::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1426;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta475(t78000: f64, t78019: f64, t78082: f64, t78112: f64, t1147: f64, t1156: f64, t1164: f64, t18915: f64, t6098: f64, t22222: f64, t4869: f64, t6085: f64, t6105: f64, t4861: f64, t72062: f64, t5988: f64, t11277: f64, t43969: f64, t50834: f64, t71335: f64, t71337: f64, t77959: f64, t77963: f64, t77967: f64, t77971: f64, t77975: f64, t77979: f64, t77983: f64, t77989: f64, t77992: f64, t77995: f64, t77998: f64, t63332: f64, t63334: f64, t63888: f64, t63893: f64, t63911: f64, t71142: f64, t71144: f64, t71146: f64, t71152: f64, t71154: f64, t71156: f64, t71408: f64, t78002: f64, t78005: f64, t44027: f64, t50846: f64, t71470: f64, t71472: f64, t71474: f64, t78026: f64, t78029: f64, t78033: f64, t78037: f64, t78041: f64, t78045: f64, t78049: f64, t78078: f64, t78080: f64, t44053: f64, t63361: f64, t78057: f64, t78084: f64, t78087: f64, t78090: f64, t78093: f64, t78095: f64, t78097: f64, t78100: f64, t78103: f64, t78105: f64, t78107: f64, t78109: f64, t1099: f64, t1118: f64, t44075: f64, t44077: f64, t43942: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t78114, t78118, t78120, t78122, t78125) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1420(t78000, t78019, t78082, t78112, t1147, t1156, t1164, t18915, t6098, t22222, t4869, t6085, t6105);
        let (t78128, t78129, t78132, t78147) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1421(t1164, t4861, t72062, t5988, t11277, t43969, t50834, t71335, t71337, t77959, t77963, t77967, t77971, t77975, t77979, t77983, t77989, t77992, t77995, t77998);
        let t78162 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1422(t63332, t63334, t63888, t63893, t63911, t71142, t71144, t71146, t71152, t71154, t71156, t71408, t78002, t78005);
        let t78177 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1423(t44027, t50846, t71470, t71472, t71474, t78026, t78029, t78033, t78037, t78041, t78045, t78049, t78078, t78080);
        let t78191 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1424(t44053, t63361, t78057, t78084, t78087, t78090, t78093, t78095, t78097, t78100, t78103, t78105, t78107, t78109);
        let (t78196, t78199, t78211) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1425(t1099, t1118, t78147, t78162, t78177, t78191, t44075, t44077, t78129, t63332, t63334, t63361, t71142, t71144, t71146, t71152, t77989, t77992, t77995, t78057);
        let t78223 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1426(t43942, t50834, t71154, t71156, t77998, t78002, t78005, t78033, t78037, t78041, t78045, t78049);
    (t78114, t78118, t78120, t78122, t78125, t78128, t78129, t78132, t78196, t78199, t78211, t78223)
}
