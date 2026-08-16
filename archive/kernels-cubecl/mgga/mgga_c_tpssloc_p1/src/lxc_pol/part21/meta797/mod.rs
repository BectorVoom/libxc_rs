//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta797 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2765;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2766;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2767;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2768;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2769;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2770;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2771;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta797<F: Float>(t16752: F, t252: F, t4233: F, t232: F, t13170: F, t2632: F, t829: F, t13397: F, t13453: F, t16758: F, t16805: F, t16815: F, t16816: F, t17030: F, t17037: F, t2684: F, t40951: F, t4162: F, t4182: F, t4280: F, t4281: F, t4282: F, t4283: F, t4291: F, t58166: F, t812: F, t860: F, t863: F, t9632: F, t13396: F, t1499: F, t13380: F, t13398: F, t13414: F, t13423: F, t13448: F, t16673: F, t16679: F, t16935: F, t2617: F, t2729: F, t2733: F, t2736: F, t40895: F, t4166: F, t4234: F, t5585: F, t5645: F, t58204: F, t9612: F, t828: F, t16841: F, t46741: F, t17017: F, t9638: F, t41107: F, t5593: F, t16914: F, t17009: F, t13084: F, t16836: F, t16839: F, t16891: F, t16896: F, t16898: F, t16901: F, t2633: F, t2643: F, t2645: F, t2679: F, t40966: F, t40982: F, t40990: F, t4178: F, t4180: F, t4240: F, t47044: F, t9642: F, t9646: F, t9647: F, t41115: F, t13258: F, t16932: F, t16937: F, t10007: F, t13080: F, t13176: F, t13244: F, t13248: F, t13251: F, t13254: F, t13262: F, t13322: F, t16845: F, t16907: F, t41123: F, t4177: F, t4181: F, t4184: F, t46546: F, t46737: F, t16893: F, t16918: F, t4191: F, t46657: F, t120: F, t13076: F, t13171: F, t13326: F, t16662: F, t16976: F, t2707: F, t41448: F, t46549: F, t46551: F, t5624: F, t9990: F, t16924: F, t17004: F, t2563: F, t12971: F, t13191: F, t13222: F, t13229: F, t13242: F, t13263: F, t13333: F, t16903: F, t16912: F, t17013: F, t20986: F, t41467: F, t4248: F, t46558: F, t46573: F, t46577: F, t46628: F, t47307: F, t58246: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t58262, t58280, t58281, t58289, t58304) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2765::<F>(t16752, t252, t4233, t232, t13170, t2632, t829, t13397, t13453, t16758, t16805, t16815, t16816, t17030, t17037, t2684, t40951, t4162, t4182, t4280, t4281, t4282, t4283, t4291, t58166, t812, t860, t863, t9632);
        let t58337 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2766::<F>(t13396, t1499, t13380, t13398, t13414, t13423, t13448, t16673, t16679, t16935, t2617, t2729, t2733, t2736, t40895, t4166, t4182, t4234, t4281, t4291, t5585, t5645, t58204, t812, t9612);
        let (t58340, t58345, t58353, t58363, t58373, t58379, t58381) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2767::<F>(t2632, t58280, t16935, t828, t16841, t46741, t17017, t9638, t41107, t5593, t16914, t17009);
        let t58392 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2768::<F>(t13084, t16836, t16839, t16891, t16896, t16898, t16901, t2633, t2643, t2645, t2679, t2684, t40966, t40982, t40990, t4178, t4180, t4240, t47044, t58353, t58363, t58373, t58379, t58381, t9642, t9646, t9647);
        let t58439 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2769::<F>(t41115, t5593, t13258, t16932, t16937, t10007, t13080, t13176, t13244, t13248, t13251, t13254, t13262, t13322, t16836, t16839, t16841, t16845, t16907, t16914, t2643, t2645, t40951, t41123, t4177, t4178, t4180, t4181, t4184, t46546, t46737, t58289, t9632, t9642);
        let t58486 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2770::<F>(t16898, t9638, t13258, t16893, t16918, t4191, t46657, t4240, t120, t13076, t13171, t13251, t13326, t16662, t16839, t16896, t16901, t16976, t17009, t2643, t2645, t2679, t2684, t2707, t41448, t4178, t4180, t4181, t46549, t46551, t5624, t829, t9642, t9646, t9990);
        let (t58495, t58540) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2771::<F>(t120, t16752, t16924, t9638, t17004, t2563, t12971, t13191, t13222, t13229, t13242, t13262, t13263, t13333, t16836, t16839, t16891, t16903, t16912, t17013, t17017, t20986, t232, t2643, t2645, t2679, t41467, t4178, t4180, t4181, t4248, t46558, t46573, t46577, t46628, t47307, t58246, t829, t9642);
    (t58262, t58281, t58304, t58337, t58340, t58345, t58392, t58439, t58486, t58495, t58540)
}
