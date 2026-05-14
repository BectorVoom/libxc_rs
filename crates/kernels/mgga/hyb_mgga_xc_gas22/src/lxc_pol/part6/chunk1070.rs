//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1070/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1070<F: Float>(t2601: F, t4323: F, t1005: F, t1006: F, t11003: F, t10898: F, t10913: F, t6969: F, t7183: F, t9008: F, t9037: F, t7176: F, t9264: F, t361: F, t10964: F, t11030: F, t11034: F, t11037: F, t11041: F, t2572: F, t2599: F, t3560: F, t3580: F, t374: F, t4311: F, t4324: F, t4327: F, t7104: F, t7133: F, t998: F) -> (F, F, F, F, F, F, F) {
    let t11139 = t4323 * t2601;
    let t11140 = t11139 * t1005;
    let t11149 = t11003 * t1006;
    let t11159 = -t7183 + 0.22831111111111111111e-1 * t6969 + 0.45662222222222222221e-1 * t9008 - t9037 - 0.17123333333333333333e-1 * t10898 + 0.5137e-1 * t10913;
    let t11166 = -t7176 + 0.23744444444444444444e-1 * t6969 + 0.47488888888888888888e-1 * t9008 - t9264 - 0.17808333333333333333e-1 * t10898 + 0.53425e-1 * t10913;
    let t11168 = 0.621814e-1 * t11166 * t361;
    let t11169 = t11030 - t11034 - t11037 - t11041 + 0.17315859105681463759e2 * t2599 * t11140 + 0.11696447245269292414e1 * t3560 * t3580 - 0.11696447245269292414e1 * t7133 * t4311 + 0.5848223622634646207e0 * t2572 * t4324 + 0.5848223622634646207e0 * t998 * t11149 + 0.17315859105681463759e2 * t7104 * t4327 - 0.19751673498613801407e-1 * t10964 - 0.310907e-1 * t11159 * t374 + t11168;
    (t11139, t11140, t11149, t11159, t11166, t11168, t11169)
}
