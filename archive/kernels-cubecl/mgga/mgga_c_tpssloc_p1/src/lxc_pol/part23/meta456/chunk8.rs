//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1327/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1327<F: Float>(t5636: F, t13397: F, t1492: F, t1510: F, t1523: F, t1528: F, t16673: F, t16758: F, t16815: F, t16830: F, t17034: F, t17052: F, t17090: F, t17092: F, t20806: F, t20862: F, t20867: F, t20873: F, t20986: F, t21013: F, t21025: F, t21028: F, t21034: F, t21050: F, t259: F, t2728: F, t40890: F, t4147: F, t4166: F, t4268: F, t4281: F, t4291: F, t5612: F, t5637: F, t5648: F, t5651: F, t5658: F, t67305: F, t67339: F, t67344: F, t67392: F, t67405: F, t67429: F, t67441: F, t68246: F, t76002: F, t76074: F, t76274: F, t76327: F, t76414: F, t76467: F, t812: F, t855: F, t858: F, t860: F) -> F {
    let t76482 = t5636 * t5636;
    let t76497 = F::cast_from(24.0_f64) * t17092 * t5637 - t855 * t858 * (-F::cast_from(36.0_f64) * t13397 * t16815 * t68246 - F::cast_from(4.0_f64) * t4291 * t67392 * t1510 + F::cast_from(24.0_f64) * t4281 * t16758 * t20986 + F::cast_from(36.0_f64) * t4281 * t16815 * t20986 - F::cast_from(6.0_f64) * t4291 * t16815 * t5612 + F::cast_from(6.0_f64) * t812 * t2728 * t76002 - t812 * t860 * t76074 - F::cast_from(6.0_f64) * t16673 * t5651 + F::cast_from(24.0_f64) * t17034 * t21025 - F::cast_from(12.0_f64) * t4166 * t20806 + t76414 - F::cast_from(12.0_f64) * t4291 * t67405 * t1510 - F::cast_from(4.0_f64) * t812 * t67429 * t1510 - F::cast_from(3.0_f64) * t812 * t860 * t76274 - t812 * t860 * t76327 - F::cast_from(4.0_f64) * t67441 * t1523 - F::cast_from(12.0_f64) * t16673 * t5648 - F::cast_from(12.0_f64) * t16830 * t20873 + F::cast_from(24.0_f64) * t4166 * t20862 + F::cast_from(24.0_f64) * t4166 * t20867 - F::cast_from(12.0_f64) * t4166 * t21028 + t76467) + F::cast_from(12.0_f64) * t17052 * t5637 - F::cast_from(24.0_f64) * t4147 * t21050 - F::cast_from(12.0_f64) * t17092 * t5658 - F::cast_from(4.0_f64) * t4147 * t21034 - F::cast_from(12.0_f64) * t67339 * t1528 + F::cast_from(24.0_f64) * t855 * t40890 * t76482 - F::cast_from(6.0_f64) * t17090 * t5658 - F::cast_from(12.0_f64) * t67305 * t1528 - F::cast_from(4.0_f64) * t67344 * t1528 - F::cast_from(4.0_f64) * t4268 * t21034 + F::cast_from(4.0_f64) * t1492 * t21013 * t259;
    t76497
}
