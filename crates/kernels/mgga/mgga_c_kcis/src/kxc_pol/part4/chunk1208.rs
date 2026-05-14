//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1208/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1208<F: Float>(t1572: F, t17885: F, t1347: F, t5586: F, t1563: F, t6072: F, t1911: F, t3918: F, t1564: F, t1573: F, t1578: F, t16105: F, t16117: F, t16119: F, t16122: F, t16124: F, t17828: F, t17831: F, t17834: F, t4326: F, t4333: F, t4356: F, t4367: F, t4373: F, t6098: F) -> (F,) {
    let t17886 = t17885 * t1572;
    let t17889 = t5586 * t1347;
    let t17892 = t6072 * t1563;
    let t17895 = t1911 * t3918;
    let t17898 = 6.0 * t4356 * t17828 + 0.35089340384731224426e1 * t4373 * t17831 - 2.0 * t17834 * t4333 + 2.0 * t4326 * t6098 + 1.0 * t1564 * t17886 + 0.11696446794910408142e1 * t17889 * t1578 + 2.0 * t17892 * t1573 + t16105 - 0.11696446794910408142e1 * t17895 * t4367 - t16117 - t16119 - t16122 - t16124;
    (t17898,)
}
