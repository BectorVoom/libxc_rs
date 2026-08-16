//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 849/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk849(t252: f64, t2678: f64, t829: f64, t860: f64, t9661: f64, t10016: f64, t10055: f64, t10058: f64, t10069: f64, t10073: f64, t10077: f64, t10081: f64, t10084: f64, t10091: f64, t10094: f64, t226: f64, t255: f64, t2613: f64, t2617: f64, t2729: f64, t2733: f64, t2736: f64, t2738: f64, t2740: f64, t4281: f64, t4291: f64, t808: f64, t812: f64, t861: f64, t863: f64, t9612: f64) -> (f64, f64, f64, f64) {
    let t10097 = t252 * t2678;
    let t10098 = t10097 * t829;
    let t10101 = t860 * t9661;
    let t10103 = t10016 * t255 + 6.0_f64 * t10055 * t812 + t10058 * t226 - t10069 * t812 - 3.0_f64 * t10073 * t812 - 3.0_f64 * t10077 * t812 - 6.0_f64 * t10081 * t812 + 6.0_f64 * t10084 * t812 - 3.0_f64 * t10091 * t812 + 6.0_f64 * t10094 * t4281 - 3.0_f64 * t10098 * t4291 - t10101 * t812 + 3.0_f64 * t2613 * t863 + 6.0_f64 * t2617 * t2729 - 6.0_f64 * t2617 * t2733 - 3.0_f64 * t2617 * t2736 - 3.0_f64 * t2617 * t2738 + 3.0_f64 * t2740 * t808 - 3.0_f64 * t861 * t9612;
    (t10097, t10098, t10101, t10103)
}
