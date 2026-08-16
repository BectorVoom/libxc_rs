//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1203/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1203(t4731: f64, t962: f64, t1684: f64, t3031: f64, t14026: f64, t971: f64, t1823: f64, t3549: f64, t10869: f64, t10888: f64, t1212: f64, t1225: f64, t1226: f64, t13854: f64, t15369: f64, t15423: f64, t15442: f64, t1831: f64, t1835: f64, t3545: f64, t3552: f64, t3578: f64, t3582: f64, t3589: f64, t3593: f64, t405: f64, t5234: f64, t5242: f64, t5250: f64) -> f64 {
    let t15445 = t4731 * t962;
    let t15450 = t1684 * t3031;
    let t15457 = t14026 * t971;
    let t15460 = t1823 * t3549;
    let t15463 = 0.32164683177870697974e2_f64 * t15369 * t3578 + 1.0_f64 * t10888 * t1831 + 2.0_f64 * t3545 * t5234 + 1.0_f64 * t1212 * t15423 - 0.3109e-1_f64 * t15442 * t405 + 0.11696446794910408142e1_f64 * t15445 * t1226 + 0.58482233974552040708e0_f64 * t5242 * t3589 + 0.17315755899375863299e2_f64 * t15450 * t3593 + 0.58482233974552040708e0_f64 * t10869 * t1835 + 0.11696446794910408142e1_f64 * t3582 * t5250 + 0.58482233974552040708e0_f64 * t1225 * t15457 - 2.0_f64 * t15460 * t3552 + t13854;
    t15463
}
