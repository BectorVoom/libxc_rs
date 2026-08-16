//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta719 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2327;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2328;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2329;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2330;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2331;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2332;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2333;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta719(t59039: f64, t16717: f64, t58994: f64, t59045: f64, t59048: f64, t39658: f64, t46436: f64, t46438: f64, t67494: f64, t67495: f64, t67496: f64, t67497: f64, t67498: f64, t67499: f64, t67500: f64, t67501: f64, t67502: f64, t67503: f64, t13151: f64, t13156: f64, t13160: f64, t1504: f64, t16662: f64, t16736: f64, t16749: f64, t16949: f64, t20756: f64, t20800: f64, t20843: f64, t20846: f64, t20849: f64, t228: f64, t4119: f64, t4225: f64, t4226: f64, t5544: f64, t6589: f64, t67282: f64, t776: f64, t822: f64, t824: f64, t845: f64, t1506: f64, t16723: f64, t16729: f64, t16737: f64, t16740: f64, t16746: f64, t20835: f64, t225: f64, t230: f64, t232: f64, t4219: f64, t4227: f64, t4230: f64, t5601: f64, t5605: f64, t5608: f64, t67448: f64, t67449: f64, t67451: f64, t67452: f64, t67455: f64, t67467: f64, t67491: f64, t68: f64, t825: f64, t4233: f64, t9975: f64, t13397: f64, t1510: f64, t16679: f64, t16815: f64, t16816: f64, t16828: f64, t16830: f64, t16935: f64, t17027: f64, t17028: f64, t20806: f64, t2617: f64, t4166: f64, t4234: f64, t4281: f64, t59347: f64, t67358: f64, t67441: f64, t812: f64, t860: f64, t861: f64, t40933: f64, t828: f64, t120: f64, t20856: f64, t46657: f64, t5593: f64, t20852: f64, t13258: f64, t20983: f64, t16839: f64, t16841: f64, t2643: f64, t4178: f64, t4180: f64, t4182: f64, t47307: f64, t58353: f64, t58363: f64, t58373: f64, t58379: f64, t58381: f64, t58904: f64, t829: f64, t20974: f64, t9638: f64, t20891: f64, t20904: f64, t41414: f64, t13177: f64, t13251: f64, t16673: f64, t16898: f64, t2645: f64, t40966: f64, t40971: f64, t4177: f64, t4184: f64, t4250: f64, t46546: f64, t5619: f64, t58421: f64, t58425: f64, t58427: f64, t58642: f64, t820: f64, t843: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t67504, t67506, t67507, t67508, t67509) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2327(t59039, t16717, t58994, t59045, t59048, t39658, t46436, t46438, t67494, t67495, t67496, t67497, t67498, t67499, t67500, t67501, t67502, t67503);
        let t67566 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2328(t13151, t13156, t13160, t1504, t16662, t16736, t16749, t16949, t20756, t20800, t20843, t20846, t20849, t228, t4119, t4225, t4226, t5544, t6589, t67282, t776, t822, t824, t845);
        let t67568 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2329(t1506, t16723, t16729, t16737, t16740, t16746, t20835, t225, t230, t232, t4219, t4227, t4230, t5601, t5605, t5608, t67448, t67449, t67451, t67452, t67455, t67467, t67491, t67509, t67566, t68, t825);
        let (t67578, t67582) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2330(t4233, t9975, t13397, t1510, t16679, t16815, t16816, t16828, t16830, t16935, t17027, t17028, t20806, t2617, t4166, t4234, t4281, t59347, t67358, t67441, t67568, t812, t860, t861);
        let (t67596, t67607) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2331(t40933, t828, t120, t20856);
        let (t67620, t67636) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2332(t46657, t5593, t120, t20852, t13258, t20983, t16839, t16841, t2643, t4178, t4180, t4182, t4234, t47307, t58353, t58363, t58373, t58379, t58381, t58904, t67596, t67607, t829);
        let t67667 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2333(t20974, t9638, t20891, t120, t20800, t20904, t41414, t13177, t13251, t16673, t16898, t20756, t2643, t2645, t40966, t40971, t4177, t4184, t4250, t46546, t5619, t58421, t58425, t58427, t58642, t776, t820, t829, t843);
    (t67504, t67506, t67507, t67508, t67568, t67578, t67582, t67596, t67607, t67620, t67636, t67667)
}
