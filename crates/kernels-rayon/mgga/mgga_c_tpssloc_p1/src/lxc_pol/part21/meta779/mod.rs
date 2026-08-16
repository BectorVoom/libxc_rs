//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta779 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2702;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2703;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2704;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2705;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta779(t5303: f64, t53945: f64, t16336: f64, t5310: f64, t5286: f64, t3792: f64, t1827: f64, t54124: f64, t16288: f64, t5289: f64, t19805: f64, t68: f64, t1340: f64, t12365: f64, t6417: f64, t12283: f64, t19962: f64, t19882: f64, t19996: f64, t3866: f64, t40018: f64, t6371: f64, t119: f64, t12351: f64, t12419: f64, t12420: f64, t1343: f64, t1354: f64, t1363: f64, t16321: f64, t19871: f64, t210: f64, t3733: f64, t3734: f64, t3790: f64, t3803: f64, t54151: f64, t54191: f64, t54198: f64, t56486: f64, t6347: f64, t820: f64, t12189: f64, t6375: f64, t40138: f64, t6396: f64, t19951: f64, t19991: f64, t40281: f64, t12407: f64, t12429: f64, t16224: f64, t16225: f64, t16305: f64, t16306: f64, t16311: f64, t16366: f64, t16370: f64, t16394: f64, t19921: f64, t19926: f64, t19976: f64, t19981: f64, t19989: f64, t3783: f64, t3805: f64, t5246: f64, t53973: f64, t54013: f64, t54162: f64, t54202: f64, t12339: f64, t6427: f64, t6431: f64, t12345: f64, t19815: f64, t3865: f64, t1369: f64, t1362: f64, t19904: f64, t3870: f64, t3872: f64, t3876: f64, t40006: f64, t40008: f64, t40019: f64, t40060: f64, t54213: f64, t54220: f64, t54222: f64, t54237: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t56906, t56909, t56913, t56914, t56919, t56921, t56923) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2702(t5303, t53945, t16336, t5310, t5286, t3792, t1827, t54124, t16288, t5289, t19805, t68);
        let t56952 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2703(t1340, t56923, t12365, t6417, t12283, t19962, t19882, t19996, t3866, t40018, t6371, t119, t12351, t12419, t12420, t1343, t1354, t1363, t16321, t19871, t210, t3733, t3734, t3790, t3803, t5310, t54151, t54191, t54198, t56486, t56906, t56909, t56914, t56919, t56921, t6347, t820);
        let t56996 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2704(t12189, t6375, t40138, t6396, t12283, t19951, t19991, t40281, t12407, t12429, t16224, t16225, t16305, t16306, t16311, t16366, t16370, t16394, t19871, t19921, t19926, t19976, t19981, t19989, t3783, t3803, t3805, t5246, t5303, t53973, t54013, t54162, t54202);
        let t57030 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2705(t12339, t6427, t6431, t12345, t19815, t3865, t1369, t1362, t56923, t1363, t19904, t3870, t3872, t3876, t40006, t40008, t40019, t40060, t54213, t54220, t54222, t54237, t56486, t820);
    (t56913, t56914, t56923, t56952, t56996, t57030)
}
