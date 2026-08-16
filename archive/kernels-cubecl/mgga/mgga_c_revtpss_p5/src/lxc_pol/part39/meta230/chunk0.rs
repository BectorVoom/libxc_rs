//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 897/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk897<F: Float>(t300: F, t5188: F, t5156: F, t1749: F, t1198: F, t1765: F, t3531: F, t1756: F, t3495: F, t1189: F, t1196: F, t1179: F, t1188: F, t5180: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5189 = t300 * t5188;
    let t5191 = F::cast_from(0.19751673498613801407e-1_f64) * t300 * t5156;
    let t5192 = t300 * t1749;
    let t5194 = F::cast_from(0.5848223622634646207e0_f64) * t5192 * t1198;
    let t5196 = F::cast_from(0.5848223622634646207e0_f64) * t3531 * t1765;
    let t5197 = t3495 * t1756;
    let t5198 = t5197 * t1189;
    let t5200 = F::cast_from(0.11696447245269292414e1_f64) * t1196 * t5198;
    let t5202 = t1179 * t5180 * t1188;
    (t5189, t5191, t5192, t5194, t5196, t5197, t5198, t5200, t5202)
}
