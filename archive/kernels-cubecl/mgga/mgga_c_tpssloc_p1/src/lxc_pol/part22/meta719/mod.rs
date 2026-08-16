//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta719 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2327;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2328;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2329;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2330;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2331;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2332;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2333;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta719<F: Float>(t59039: F, t16717: F, t58994: F, t59045: F, t59048: F, t39658: F, t46436: F, t46438: F, t67494: F, t67495: F, t67496: F, t67497: F, t67498: F, t67499: F, t67500: F, t67501: F, t67502: F, t67503: F, t13151: F, t13156: F, t13160: F, t1504: F, t16662: F, t16736: F, t16749: F, t16949: F, t20756: F, t20800: F, t20843: F, t20846: F, t20849: F, t228: F, t4119: F, t4225: F, t4226: F, t5544: F, t6589: F, t67282: F, t776: F, t822: F, t824: F, t845: F, t1506: F, t16723: F, t16729: F, t16737: F, t16740: F, t16746: F, t20835: F, t225: F, t230: F, t232: F, t4219: F, t4227: F, t4230: F, t5601: F, t5605: F, t5608: F, t67448: F, t67449: F, t67451: F, t67452: F, t67455: F, t67467: F, t67491: F, t68: F, t825: F, t4233: F, t9975: F, t13397: F, t1510: F, t16679: F, t16815: F, t16816: F, t16828: F, t16830: F, t16935: F, t17027: F, t17028: F, t20806: F, t2617: F, t4166: F, t4234: F, t4281: F, t59347: F, t67358: F, t67441: F, t812: F, t860: F, t861: F, t40933: F, t828: F, t120: F, t20856: F, t46657: F, t5593: F, t20852: F, t13258: F, t20983: F, t16839: F, t16841: F, t2643: F, t4178: F, t4180: F, t4182: F, t47307: F, t58353: F, t58363: F, t58373: F, t58379: F, t58381: F, t58904: F, t829: F, t20974: F, t9638: F, t20891: F, t20904: F, t41414: F, t13177: F, t13251: F, t16673: F, t16898: F, t2645: F, t40966: F, t40971: F, t4177: F, t4184: F, t4250: F, t46546: F, t5619: F, t58421: F, t58425: F, t58427: F, t58642: F, t820: F, t843: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t67504, t67506, t67507, t67508, t67509) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2327::<F>(t59039, t16717, t58994, t59045, t59048, t39658, t46436, t46438, t67494, t67495, t67496, t67497, t67498, t67499, t67500, t67501, t67502, t67503);
        let t67566 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2328::<F>(t13151, t13156, t13160, t1504, t16662, t16736, t16749, t16949, t20756, t20800, t20843, t20846, t20849, t228, t4119, t4225, t4226, t5544, t6589, t67282, t776, t822, t824, t845);
        let t67568 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2329::<F>(t1506, t16723, t16729, t16737, t16740, t16746, t20835, t225, t230, t232, t4219, t4227, t4230, t5601, t5605, t5608, t67448, t67449, t67451, t67452, t67455, t67467, t67491, t67509, t67566, t68, t825);
        let (t67578, t67582) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2330::<F>(t4233, t9975, t13397, t1510, t16679, t16815, t16816, t16828, t16830, t16935, t17027, t17028, t20806, t2617, t4166, t4234, t4281, t59347, t67358, t67441, t67568, t812, t860, t861);
        let (t67596, t67607) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2331::<F>(t40933, t828, t120, t20856);
        let (t67620, t67636) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2332::<F>(t46657, t5593, t120, t20852, t13258, t20983, t16839, t16841, t2643, t4178, t4180, t4182, t4234, t47307, t58353, t58363, t58373, t58379, t58381, t58904, t67596, t67607, t829);
        let t67667 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2333::<F>(t20974, t9638, t20891, t120, t20800, t20904, t41414, t13177, t13251, t16673, t16898, t20756, t2643, t2645, t40966, t40971, t4177, t4184, t4250, t46546, t5619, t58421, t58425, t58427, t58642, t776, t820, t829, t843);
    (t67504, t67506, t67507, t67508, t67568, t67578, t67582, t67596, t67607, t67620, t67636, t67667)
}
