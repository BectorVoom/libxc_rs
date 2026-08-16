//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1327/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1327(t5636: f64, t13397: f64, t1492: f64, t1510: f64, t1523: f64, t1528: f64, t16673: f64, t16758: f64, t16815: f64, t16830: f64, t17034: f64, t17052: f64, t17090: f64, t17092: f64, t20806: f64, t20862: f64, t20867: f64, t20873: f64, t20986: f64, t21013: f64, t21025: f64, t21028: f64, t21034: f64, t21050: f64, t259: f64, t2728: f64, t40890: f64, t4147: f64, t4166: f64, t4268: f64, t4281: f64, t4291: f64, t5612: f64, t5637: f64, t5648: f64, t5651: f64, t5658: f64, t67305: f64, t67339: f64, t67344: f64, t67392: f64, t67405: f64, t67429: f64, t67441: f64, t68246: f64, t76002: f64, t76074: f64, t76274: f64, t76327: f64, t76414: f64, t76467: f64, t812: f64, t855: f64, t858: f64, t860: f64) -> f64 {
    let t76482 = t5636 * t5636;
    let t76497 = 24.0_f64 * t17092 * t5637 - t855 * t858 * (-36.0_f64 * t13397 * t16815 * t68246 - 4.0_f64 * t4291 * t67392 * t1510 + 24.0_f64 * t4281 * t16758 * t20986 + 36.0_f64 * t4281 * t16815 * t20986 - 6.0_f64 * t4291 * t16815 * t5612 + 6.0_f64 * t812 * t2728 * t76002 - t812 * t860 * t76074 - 6.0_f64 * t16673 * t5651 + 24.0_f64 * t17034 * t21025 - 12.0_f64 * t4166 * t20806 + t76414 - 12.0_f64 * t4291 * t67405 * t1510 - 4.0_f64 * t812 * t67429 * t1510 - 3.0_f64 * t812 * t860 * t76274 - t812 * t860 * t76327 - 4.0_f64 * t67441 * t1523 - 12.0_f64 * t16673 * t5648 - 12.0_f64 * t16830 * t20873 + 24.0_f64 * t4166 * t20862 + 24.0_f64 * t4166 * t20867 - 12.0_f64 * t4166 * t21028 + t76467) + 12.0_f64 * t17052 * t5637 - 24.0_f64 * t4147 * t21050 - 12.0_f64 * t17092 * t5658 - 4.0_f64 * t4147 * t21034 - 12.0_f64 * t67339 * t1528 + 24.0_f64 * t855 * t40890 * t76482 - 6.0_f64 * t17090 * t5658 - 12.0_f64 * t67305 * t1528 - 4.0_f64 * t67344 * t1528 - 4.0_f64 * t4268 * t21034 + 4.0_f64 * t1492 * t21013 * t259;
    t76497
}
