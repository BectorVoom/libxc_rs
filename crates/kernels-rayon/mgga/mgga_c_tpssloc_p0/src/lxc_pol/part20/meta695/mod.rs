//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta695 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2647;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2648;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2649;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2650;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2651;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2652;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2653;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta695(t16060: f64, t3865: f64, t1369: f64, t16123: f64, t68: f64, t1362: f64, t1831: f64, t40292: f64, t12345: f64, t5314: f64, t12211: f64, t16296: f64, t40018: f64, t5223: f64, t16379: f64, t40021: f64, t12156: f64, t12240: f64, t12353: f64, t12407: f64, t1307: f64, t16225: f64, t16305: f64, t16306: f64, t16321: f64, t16355: f64, t1810: f64, t210: f64, t3733: f64, t3803: f64, t3876: f64, t39936: f64, t40025: f64, t5240: f64, t5246: f64, t12282: f64, t5234: f64, t3809: f64, t120: f64, t16205: f64, t12283: f64, t16227: f64, t1351: f64, t5286: f64, t12429: f64, t1352: f64, t16148: f64, t16224: f64, t16308: f64, t16311: f64, t16314: f64, t16401: f64, t3805: f64, t3807: f64, t39945: f64, t39948: f64, t39950: f64, t39956: f64, t39958: f64, t39960: f64, t40197: f64, t12189: f64, t5227: f64, t16232: f64, t3777: f64, t40281: f64, t5303: f64, t16300: f64, t5247: f64, t820: f64, t12250: f64, t1824: f64, t3850: f64, t12290: f64, t3789: f64, t12012: f64, t12215: f64, t12293: f64, t12303: f64, t12420: f64, t16048: f64, t16233: f64, t16235: f64, t16242: f64, t1825: f64, t3719: f64, t3734: f64, t3795: f64, t39971: f64, t5226: f64, t5248: f64, t16288: f64, t3853: f64, t12384: f64, t5293: f64, t12397: f64, t1363: f64, t16257: f64, t16271: f64, t16275: f64, t16278: f64, t1799: f64, t1827: f64, t3858: f64, t39973: f64, t39975: f64, t39983: f64, t39989: f64, t40070: f64, t40119: f64, t5289: f64, t16405: f64, t40167: f64, t3791: f64, t40138: f64, t5259: f64, t16248: f64, t12178: f64, t16018: f64, t16364: f64, t16370: f64, t16387: f64, t16391: f64, t3793: f64, t5249: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t53907, t53909, t53910, t53918, t53920, t53921) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2647(t16060, t3865, t1369, t16123, t68, t1362, t1831, t40292, t12345, t5314, t12211, t16296);
        let t53943 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2648(t40018, t5223, t16379, t40021, t12156, t12240, t12353, t12407, t1307, t1369, t16225, t16305, t16306, t16321, t16355, t1810, t210, t3733, t3803, t3876, t39936, t40025, t5240, t5246, t53907, t53910, t53918, t53920, t53921);
        let (t53958, t53978) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2649(t12282, t5234, t3809, t120, t16205, t12283, t16227, t1351, t5286, t12429, t1352, t16148, t16224, t16305, t16308, t16311, t16314, t16401, t3803, t3805, t3807, t39945, t39948, t39950, t39956, t39958, t39960, t40197, t5246);
        let (t53985, t53990, t53998, t54003, t54013, t54014) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2650(t12189, t5227, t16232, t3777, t40281, t5303, t12211, t16300, t5247, t820, t12250, t1824);
        let (t54015, t54026) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2651(t1351, t3850, t12290, t5234, t16060, t3789, t12012, t12215, t12293, t12303, t12420, t16048, t16224, t16233, t16235, t16242, t16306, t1810, t1825, t210, t3719, t3733, t3734, t3795, t3803, t39971, t5226, t5248, t53985, t53990, t53998, t54003, t54013, t54014);
        let t54058 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2652(t16288, t3853, t12384, t5234, t3795, t40281, t5293, t12156, t12397, t12429, t1363, t16257, t16271, t16275, t16278, t16401, t1799, t1827, t3858, t39973, t39975, t39983, t39989, t40070, t40119, t5289, t820);
        let (t54068, t54100) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2653(t12283, t16405, t40167, t820, t1799, t3791, t40138, t5259, t16248, t5293, t16275, t120, t12178, t12420, t12429, t1352, t16018, t16224, t16225, t16227, t16364, t16370, t16387, t16391, t16401, t3793, t3803, t3805, t3807, t5246, t5248, t5249);
    (t53909, t53943, t53958, t53978, t54013, t54014, t54015, t54026, t54058, t54068, t54100)
}
