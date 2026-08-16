//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 996/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk996(t300: f64, t5188: f64, t5156: f64, t1749: f64, t1198: f64, t1765: f64, t3531: f64, t1756: f64, t3495: f64, t1189: f64, t1196: f64, t1179: f64, t1188: f64, t5180: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5189 = t300 * t5188;
    let t5191 = 0.19751673498613801407e-1_f64 * t300 * t5156;
    let t5192 = t300 * t1749;
    let t5194 = 0.5848223622634646207e0_f64 * t5192 * t1198;
    let t5196 = 0.5848223622634646207e0_f64 * t3531 * t1765;
    let t5197 = t3495 * t1756;
    let t5198 = t5197 * t1189;
    let t5200 = 0.11696447245269292414e1_f64 * t1196 * t5198;
    let t5202 = t1179 * t5180 * t1188;
    (t5189, t5191, t5192, t5194, t5196, t5197, t5198, t5200, t5202)
}
