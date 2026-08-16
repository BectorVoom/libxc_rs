//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta450 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1808;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1809;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1810;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta450(t120: f64, t6347: f64, t1352: f64, t3805: f64, t5187: f64, t550: f64, t5249: f64, t1307: f64, t3870: f64, t820: f64, t19744: f64, t19871: f64, t5248: f64, t12369: f64, t12346: f64, t12366: f64, t12429: f64, t1363: f64, t16233: f64, t16394: f64, t16400: f64, t19940: f64, t19942: f64, t19945: f64, t19951: f64, t19958: f64, t19962: f64, t19966: f64, t19972: f64, t19976: f64, t19981: f64, t3803: f64, t5246: f64, t5259: f64, t6396: f64, t19862: f64, t19899: f64, t19939: f64, t553: f64, t5287: f64, t5335: f64, t19739: f64, t1332: f64, t1336: f64, t1381: f64, t1383: f64, t16060: f64, t1814: f64, t1838: f64, t1840: f64, t19756: f64, t19761: f64, t19763: f64, t19805: f64, t19810: f64, t19813: f64, t19815: f64, t5230: f64, t5234: f64, t5339: f64, t5341: f64, t5344: f64, t5345: f64, t5351: f64, t544: f64, t564: f64, t6378: f64, t6458: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19986, t19989, t19991, t19994) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1808(t120, t6347, t1352, t3805, t5187, t550, t5249, t1307);
        let (t19996, t20000, t20004, t20007) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1809(t19994, t3870, t820, t19744, t19871, t5248, t12369, t3805, t12346, t12366, t12429, t1363, t16233, t16394, t16400, t19940, t19942, t19945, t19951, t19958, t19962, t19966, t19972, t19976, t19981, t19986, t19991, t3803, t5246, t5259, t6396);
        let (t20009, t20010, t20014, t20018, t20021) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1810(t19862, t19899, t19939, t20007, t553, t5287, t5335, t1352, t19739, t1332, t1336, t1381, t1383, t16060, t1814, t1838, t1840, t19756, t19761, t19763, t19805, t19810, t19813, t19815, t5230, t5234, t5339, t5341, t5344, t5345, t5351, t544, t564, t6378, t6458);
    (t19986, t19989, t19991, t19994, t19996, t20000, t20004, t20009, t20010, t20014, t20018, t20021)
}
