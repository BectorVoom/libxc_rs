//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 632/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk632(t1737: f64, t1746: f64, t8763: f64, t4954: f64, t8746: f64, t4957: f64, t1706: f64, t1735: f64, t2418: f64, t2432: f64, t45: f64, t4858: f64, t4909: f64, t621: f64, t634: f64, t7091: f64, t7151: f64, t8692: f64, t8698: f64, t8730: f64, t8733: f64, t8740: f64, t8748: f64) -> (f64, f64, f64) {
    let t8765 = t1737 * t8763 * t1746;
    let t8768 = t4954 * t8746;
    let t8769 = t8768 * t4957;
    let t8772 = -0.62182e-1_f64 * t8692 * t621 + 2.0_f64 * t7091 * t2418 - 2.0_f64 * t4858 * t8698 + 1.0_f64 * t1706 * t8730 + 0.16081824322151104822e2_f64 * t4909 * t8733 + 0.19751789702565206229e-1_f64 * t45 * t8740 * t634 - 0.11696446794910408142e1_f64 * t7151 * t2432 + 0.11696446794910408142e1_f64 * t1735 * t8748 - 0.58482233974552040708e0_f64 * t1735 * t8765 - 0.17315755899375863299e2_f64 * t1735 * t8769;
    (t8765, t8769, t8772)
}
