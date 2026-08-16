//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta857 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3108;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3109;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3110;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3111;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3112;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3113;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3114;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta857(t43780: f64, t43782: f64, t43816: f64, t43895: f64, t50968: f64, t50970: f64, t50972: f64, t50978: f64, t51039: f64, t51041: f64, t64028: f64, t64031: f64, t64033: f64, t64042: f64, t64045: f64, t51043: f64, t51051: f64, t51053: f64, t63355: f64, t63359: f64, t63361: f64, t63365: f64, t63370: f64, t63374: f64, t63380: f64, t63382: f64, t63384: f64, t63388: f64, t63392: f64, t63396: f64, t63398: f64, t63400: f64, t63404: f64, t63408: f64, t63412: f64, t63417: f64, t63422: f64, t64074: f64, t64076: f64, t64079: f64, t64082: f64, t64085: f64, t64087: f64, t64089: f64, t64092: f64, t64309: f64, t64325: f64, t64342: f64, t64358: f64, t64374: f64, t1117: f64, t51460: f64, t51638: f64, t3313: f64, t3315: f64, t63287: f64, t15061: f64, t50819: f64, t11361: f64, t11365: f64, t1137: f64, t11420: f64, t1148: f64, t1155: f64, t1156: f64, t15126: f64, t15136: f64, t15146: f64, t15179: f64, t15219: f64, t15229: f64, t18603: f64, t3332: f64, t3333: f64, t3334: f64, t3357: f64, t3359: f64, t3377: f64, t3401: f64, t44188: f64, t4840: f64, t4862: f64, t51371: f64, t51385: f64, t51651: f64, t51677: f64, t6037: f64, t6053: f64, t6069: f64, t6085: f64, t6088: f64, t64261: f64, t64292: f64, t300: f64, t63457: f64, t63506: f64, t63561: f64, t63611: f64, t63715: f64, t63760: f64, t64260: f64, t1254: f64, t5091: f64, t11282: f64, t6084: f64, t1164: f64, t14854: f64, t18926: f64, t3411: f64, t14858: f64, t4884: f64, t18915: f64, t3419: f64, t18283: f64, t14855: f64, t4869: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t64389 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3108(t43780, t43782, t43816, t43895, t50968, t50970, t50972, t50978, t51039, t51041, t64028, t64031, t64033, t64042, t64045);
        let t64406 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3109(t51043, t51051, t51053, t63355, t63359, t63361, t63365, t63370, t63374, t63380, t63382, t63384, t63388, t63392, t63396);
        let t64422 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3110(t63398, t63400, t63404, t63408, t63412, t63417, t63422, t64074, t64076, t64079, t64082, t64085, t64087, t64089, t64092);
        let (t64425, t64433, t64436) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3111(t64309, t64325, t64342, t64358, t64374, t64389, t64406, t64422, t1117, t51460, t51638, t3313, t3315, t63287);
        let (t64441, t64442) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3112(t15061, t50819, t11361, t11365, t1137, t11420, t1148, t1155, t1156, t15126, t15136, t15146, t15179, t15219, t15229, t18603, t3332, t3333, t3334, t3357, t3359, t3377, t3401, t44188, t4840, t4862, t51371, t51385, t51651, t51677, t6037, t6053, t6069, t6085, t6088, t64261, t64292, t64425, t64433, t64436);
        let (t64446, t64447, t64451) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3113(t300, t63457, t63506, t63561, t63611, t63715, t63760, t64260, t64442, t1254, t5091, t11282, t6084);
        let (t64454, t64456, t64458, t64460, t64462, t64464) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3114(t1164, t14854, t64451, t18926, t3411, t14858, t4884, t18915, t3419, t18283, t14855, t4869);
    (t64425, t64433, t64436, t64441, t64446, t64447, t64454, t64456, t64458, t64460, t64462, t64464)
}
