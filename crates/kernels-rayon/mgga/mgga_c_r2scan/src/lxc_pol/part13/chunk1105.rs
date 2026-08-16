//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1105/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1105(t481: f64, t9577: f64, t39263: f64, t4176: f64, t792: f64, t37327: f64, t37377: f64, t37380: f64, t37387: f64, t39188: f64, t39195: f64, t39201: f64, t39205: f64, t39208: f64, t39212: f64, t39247: f64, t39252: f64, t39256: f64, t39261: f64) -> (f64, f64, f64) {
    let t39264 = t9577 * t481;
    let t39267 = 3.0_f64 * t39263 * t4176 * t39264;
    let t39268 = t9577 * t792;
    let t39271 = 15.0_f64 / 8.0_f64 * t37327 * t4176 * t39268;
    let t39272 = -0.36021158228745895953e-3_f64 * t39247 + t39252 + t39256 - t39188 - t39195 + t39201 + t39205 + 0.96056421943322389208e-3_f64 * t37377 + t39208 - 0.40650199722100037752e-3_f64 * t37380 - t39212 - t37387 - t39261 + t39267 - t39271;
    (t39267, t39271, t39272)
}
