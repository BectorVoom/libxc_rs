//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1128/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1128(t18523: f64, t457: f64, t460: f64, t974: f64, t135: f64, t6146: f64, t1174: f64, t6140: f64, t11558: f64, t15341: f64, t15364: f64, t15366: f64, t15374: f64, t15376: f64, t18475: f64, t18484: f64, t18489: f64, t3447: f64, t4905: f64, t4909: f64, t4920: f64) -> f64 {
    let t18525 = t457 * t18523 * t460;
    let t18526 = t974 * t18525;
    let t18529 = t135 * t6146;
    let t18530 = t1174 * t18529;
    let t18532 = t135 * t6140;
    let t18533 = t1174 * t18532;
    let t18535 = 0.22222222222222222221e-2_f64 * t3447 * t18475 - 0.14814814814814814815e-2_f64 * t15376 * t4920 - 0.14814814814814814814e-2_f64 * t15376 * t4905 + 0.29629629629629629628e-2_f64 * t15376 * t4909 + t15341 - 0.74074074074074074072e-3_f64 * t3447 * t18484 + 0.37037037037037037036e-3_f64 * t15364 + 0.14814814814814814814e-2_f64 * t15366 - t15374 + 0.27160493827160493827e-2_f64 * t18489 + 0.18518518518518518518e-3_f64 * t11558 - 0.83333333333333333332e-3_f64 * t1174 * t18526 - 0.27777777777777777777e-3_f64 * t18530 - 0.27777777777777777777e-3_f64 * t18533;
    t18535
}
