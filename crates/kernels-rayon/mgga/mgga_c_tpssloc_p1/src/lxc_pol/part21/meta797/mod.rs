//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta797 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2765;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2766;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2767;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2768;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2769;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2770;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2771;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta797(t16752: f64, t252: f64, t4233: f64, t232: f64, t13170: f64, t2632: f64, t829: f64, t13397: f64, t13453: f64, t16758: f64, t16805: f64, t16815: f64, t16816: f64, t17030: f64, t17037: f64, t2684: f64, t40951: f64, t4162: f64, t4182: f64, t4280: f64, t4281: f64, t4282: f64, t4283: f64, t4291: f64, t58166: f64, t812: f64, t860: f64, t863: f64, t9632: f64, t13396: f64, t1499: f64, t13380: f64, t13398: f64, t13414: f64, t13423: f64, t13448: f64, t16673: f64, t16679: f64, t16935: f64, t2617: f64, t2729: f64, t2733: f64, t2736: f64, t40895: f64, t4166: f64, t4234: f64, t5585: f64, t5645: f64, t58204: f64, t9612: f64, t828: f64, t16841: f64, t46741: f64, t17017: f64, t9638: f64, t41107: f64, t5593: f64, t16914: f64, t17009: f64, t13084: f64, t16836: f64, t16839: f64, t16891: f64, t16896: f64, t16898: f64, t16901: f64, t2633: f64, t2643: f64, t2645: f64, t2679: f64, t40966: f64, t40982: f64, t40990: f64, t4178: f64, t4180: f64, t4240: f64, t47044: f64, t9642: f64, t9646: f64, t9647: f64, t41115: f64, t13258: f64, t16932: f64, t16937: f64, t10007: f64, t13080: f64, t13176: f64, t13244: f64, t13248: f64, t13251: f64, t13254: f64, t13262: f64, t13322: f64, t16845: f64, t16907: f64, t41123: f64, t4177: f64, t4181: f64, t4184: f64, t46546: f64, t46737: f64, t16893: f64, t16918: f64, t4191: f64, t46657: f64, t120: f64, t13076: f64, t13171: f64, t13326: f64, t16662: f64, t16976: f64, t2707: f64, t41448: f64, t46549: f64, t46551: f64, t5624: f64, t9990: f64, t16924: f64, t17004: f64, t2563: f64, t12971: f64, t13191: f64, t13222: f64, t13229: f64, t13242: f64, t13263: f64, t13333: f64, t16903: f64, t16912: f64, t17013: f64, t20986: f64, t41467: f64, t4248: f64, t46558: f64, t46573: f64, t46577: f64, t46628: f64, t47307: f64, t58246: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t58262, t58280, t58281, t58289, t58304) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2765(t16752, t252, t4233, t232, t13170, t2632, t829, t13397, t13453, t16758, t16805, t16815, t16816, t17030, t17037, t2684, t40951, t4162, t4182, t4280, t4281, t4282, t4283, t4291, t58166, t812, t860, t863, t9632);
        let t58337 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2766(t13396, t1499, t13380, t13398, t13414, t13423, t13448, t16673, t16679, t16935, t2617, t2729, t2733, t2736, t40895, t4166, t4182, t4234, t4281, t4291, t5585, t5645, t58204, t812, t9612);
        let (t58340, t58345, t58353, t58363, t58373, t58379, t58381) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2767(t2632, t58280, t16935, t828, t16841, t46741, t17017, t9638, t41107, t5593, t16914, t17009);
        let t58392 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2768(t13084, t16836, t16839, t16891, t16896, t16898, t16901, t2633, t2643, t2645, t2679, t2684, t40966, t40982, t40990, t4178, t4180, t4240, t47044, t58353, t58363, t58373, t58379, t58381, t9642, t9646, t9647);
        let t58439 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2769(t41115, t5593, t13258, t16932, t16937, t10007, t13080, t13176, t13244, t13248, t13251, t13254, t13262, t13322, t16836, t16839, t16841, t16845, t16907, t16914, t2643, t2645, t40951, t41123, t4177, t4178, t4180, t4181, t4184, t46546, t46737, t58289, t9632, t9642);
        let t58486 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2770(t16898, t9638, t13258, t16893, t16918, t4191, t46657, t4240, t120, t13076, t13171, t13251, t13326, t16662, t16839, t16896, t16901, t16976, t17009, t2643, t2645, t2679, t2684, t2707, t41448, t4178, t4180, t4181, t46549, t46551, t5624, t829, t9642, t9646, t9990);
        let (t58495, t58540) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2771(t120, t16752, t16924, t9638, t17004, t2563, t12971, t13191, t13222, t13229, t13242, t13262, t13263, t13333, t16836, t16839, t16891, t16903, t16912, t17013, t17017, t20986, t232, t2643, t2645, t2679, t41467, t4178, t4180, t4181, t4248, t46558, t46573, t46577, t46628, t47307, t58246, t829, t9642);
    (t58262, t58281, t58304, t58337, t58340, t58345, t58392, t58439, t58486, t58495, t58540)
}
