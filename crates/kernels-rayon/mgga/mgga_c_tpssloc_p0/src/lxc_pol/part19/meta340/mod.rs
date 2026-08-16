//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta340 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1209;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1210;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1211;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1212;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1213;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1214;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta340(t40961: f64, t849: f64, t10021: f64, t812: f64, t841: f64, t23076: f64, t241: f64, t67: f64, t2379: f64, t2553: f64, t2707: f64, t9601: f64, t2697: f64, t9997: f64, t9609: f64, t2703: f64, t40904: f64, t842: f64, t2623: f64, t2701: f64, t40959: f64, t820: f64, t843: f64, t9990: f64, t9573: f64, t9657: f64, t2559: f64, t2570: f64, t2606: f64, t782: f64, t9558: f64, t10033: f64, t2632: f64, t9957: f64, t9638: f64, t9653: f64, t9623: f64, t10007: f64, t10009: f64, t13350: f64, t210: f64, t2571: f64, t2605: f64, t2643: f64, t2645: f64, t2646: f64, t2684: f64, t4178: f64, t4180: f64, t804: f64, t829: f64, t9516: f64, t9559: f64, t9616: f64, t9621: f64, t9626: f64, t9642: f64, t120: f64, t9660: f64, t10003: f64, t2617: f64, t9600: f64, t9993: f64, t2642: f64, t9612: f64, t9649: f64, t13262: f64, t2649: f64, t40848: f64, t40951: f64, t847: f64, t9627: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40962, t40966, t40971, t40972, t40977) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1209(t40961, t849, t10021, t812, t841, t23076, t241, t67, t2379, t2553);
        let t40995 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1210(t2707, t9601, t2697, t9997, t9609, t2703, t40904, t842, t2623, t2701, t40959, t40962, t40966, t40971, t40972, t40977, t820, t843, t849, t9990);
        let (t40998, t41008, t41009, t41011, t41012, t41014, t41025) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1211(t9573, t9657, t2559, t2570, t2606, t782, t9558, t10033, t2632, t9957, t9638, t9653);
        let t41037 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1212(t9623, t9638, t10007, t10009, t13350, t210, t2553, t2571, t2605, t2643, t2645, t2646, t2684, t2707, t40998, t41009, t41012, t41014, t41025, t4178, t4180, t804, t829, t9516, t9559, t9616, t9621, t9626, t9642, t9990);
        let (t41039, t41048, t41050, t41053, t41055, t41063) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1213(t120, t9660, t10003, t9638, t10009, t2617, t9600, t849, t2707, t9993, t2642, t9612);
        let (t41072, t41077) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1214(t9638, t9649, t120, t9957, t13262, t2623, t2643, t2645, t2649, t40848, t40951, t41039, t41048, t41050, t41053, t41055, t41063, t4178, t4180, t820, t829, t843, t847, t9623, t9626, t9627, t9642, t9997);
    (t40972, t40977, t40995, t41008, t41011, t41037, t41039, t41072, t41077)
}
