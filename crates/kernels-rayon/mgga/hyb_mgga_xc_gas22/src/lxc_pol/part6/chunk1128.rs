//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1128/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1128(t11166: f64, t361: f64, t10964: f64, t11030: f64, t11034: f64, t11037: f64, t11041: f64, t11140: f64, t11149: f64, t11159: f64, t2572: f64, t2599: f64, t3560: f64, t3580: f64, t374: f64, t4311: f64, t4324: f64, t4327: f64, t7104: f64, t7133: f64, t998: f64) -> (f64, f64) {
    let t11168 = 0.621814e-1_f64 * t11166 * t361;
    let t11169 = t11030 - t11034 - t11037 - t11041 + 0.17315859105681463759e2_f64 * t2599 * t11140 + 0.11696447245269292414e1_f64 * t3560 * t3580 - 0.11696447245269292414e1_f64 * t7133 * t4311 + 0.5848223622634646207e0_f64 * t2572 * t4324 + 0.5848223622634646207e0_f64 * t998 * t11149 + 0.17315859105681463759e2_f64 * t7104 * t4327 - 0.19751673498613801407e-1_f64 * t10964 - 0.310907e-1_f64 * t11159 * t374 + t11168;
    (t11168, t11169)
}
