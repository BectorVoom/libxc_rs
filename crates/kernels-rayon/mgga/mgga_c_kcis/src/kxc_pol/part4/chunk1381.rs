//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1381/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1381(t1572: f64, t17885: f64, t1347: f64, t5586: f64, t1563: f64, t6072: f64, t1911: f64, t3918: f64, t1564: f64, t1573: f64, t1578: f64, t16105: f64, t16117: f64, t16119: f64, t16122: f64, t16124: f64, t17828: f64, t17831: f64, t17834: f64, t4326: f64, t4333: f64, t4356: f64, t4367: f64, t4373: f64, t6098: f64) -> f64 {
    let t17886 = t17885 * t1572;
    let t17889 = t5586 * t1347;
    let t17892 = t6072 * t1563;
    let t17895 = t1911 * t3918;
    let t17898 = 6.0_f64 * t4356 * t17828 + 0.35089340384731224426e1_f64 * t4373 * t17831 - 2.0_f64 * t17834 * t4333 + 2.0_f64 * t4326 * t6098 + 1.0_f64 * t1564 * t17886 + 0.11696446794910408142e1_f64 * t17889 * t1578 + 2.0_f64 * t17892 * t1573 + t16105 - 0.11696446794910408142e1_f64 * t17895 * t4367 - t16117 - t16119 - t16122 - t16124;
    t17898
}
