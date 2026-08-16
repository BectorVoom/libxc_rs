//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta340 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1209;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1210;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1211;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1212;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1213;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1214;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta340<F: Float>(t40961: F, t849: F, t10021: F, t812: F, t841: F, t23076: F, t241: F, t67: F, t2379: F, t2553: F, t2707: F, t9601: F, t2697: F, t9997: F, t9609: F, t2703: F, t40904: F, t842: F, t2623: F, t2701: F, t40959: F, t820: F, t843: F, t9990: F, t9573: F, t9657: F, t2559: F, t2570: F, t2606: F, t782: F, t9558: F, t10033: F, t2632: F, t9957: F, t9638: F, t9653: F, t9623: F, t10007: F, t10009: F, t13350: F, t210: F, t2571: F, t2605: F, t2643: F, t2645: F, t2646: F, t2684: F, t4178: F, t4180: F, t804: F, t829: F, t9516: F, t9559: F, t9616: F, t9621: F, t9626: F, t9642: F, t120: F, t9660: F, t10003: F, t2617: F, t9600: F, t9993: F, t2642: F, t9612: F, t9649: F, t13262: F, t2649: F, t40848: F, t40951: F, t847: F, t9627: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t40962, t40966, t40971, t40972, t40977) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1209::<F>(t40961, t849, t10021, t812, t841, t23076, t241, t67, t2379, t2553);
        let t40995 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1210::<F>(t2707, t9601, t2697, t9997, t9609, t2703, t40904, t842, t2623, t2701, t40959, t40962, t40966, t40971, t40972, t40977, t820, t843, t849, t9990);
        let (t40998, t41008, t41009, t41011, t41012, t41014, t41025) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1211::<F>(t9573, t9657, t2559, t2570, t2606, t782, t9558, t10033, t2632, t9957, t9638, t9653);
        let t41037 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1212::<F>(t9623, t9638, t10007, t10009, t13350, t210, t2553, t2571, t2605, t2643, t2645, t2646, t2684, t2707, t40998, t41009, t41012, t41014, t41025, t4178, t4180, t804, t829, t9516, t9559, t9616, t9621, t9626, t9642, t9990);
        let (t41039, t41048, t41050, t41053, t41055, t41063) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1213::<F>(t120, t9660, t10003, t9638, t10009, t2617, t9600, t849, t2707, t9993, t2642, t9612);
        let (t41072, t41077) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1214::<F>(t9638, t9649, t120, t9957, t13262, t2623, t2643, t2645, t2649, t40848, t40951, t41039, t41048, t41050, t41053, t41055, t41063, t4178, t4180, t820, t829, t843, t847, t9623, t9626, t9627, t9642, t9997);
    (t40972, t40977, t40995, t41008, t41011, t41037, t41039, t41072, t41077)
}
