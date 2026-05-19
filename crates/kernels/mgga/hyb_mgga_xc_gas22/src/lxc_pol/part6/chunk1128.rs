//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1128/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1128<F: Float>(t11166: F, t361: F, t10964: F, t11030: F, t11034: F, t11037: F, t11041: F, t11140: F, t11149: F, t11159: F, t2572: F, t2599: F, t3560: F, t3580: F, t374: F, t4311: F, t4324: F, t4327: F, t7104: F, t7133: F, t998: F) -> (F, F) {
    let t11168 = F::new(0.621814e-1) * t11166 * t361;
    let t11169 = t11030 - t11034 - t11037 - t11041 + F::cast_from(0.17315859105681463759e2_f64) * t2599 * t11140 + F::cast_from(0.11696447245269292414e1_f64) * t3560 * t3580 - F::cast_from(0.11696447245269292414e1_f64) * t7133 * t4311 + F::cast_from(0.5848223622634646207e0_f64) * t2572 * t4324 + F::cast_from(0.5848223622634646207e0_f64) * t998 * t11149 + F::cast_from(0.17315859105681463759e2_f64) * t7104 * t4327 - F::cast_from(0.19751673498613801407e-1_f64) * t10964 - F::new(0.310907e-1) * t11159 * t374 + t11168;
    (t11168, t11169)
}
