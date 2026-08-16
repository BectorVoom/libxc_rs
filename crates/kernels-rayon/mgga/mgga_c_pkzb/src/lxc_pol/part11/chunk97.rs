//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 97/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk97(t252: f64, t261: f64, t230: f64, t237: f64, t239: f64, t248: f64, t136: f64, t173: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t231 = 2.0_f64 <= zeta_threshold;
    let t234 = 0.0_f64 <= zeta_threshold;
    let t262 = t252 * t261;
    let t265 = t237 * (-0.310907e-1_f64 * t239 * t248 + t230 - 0.19751673498613801407e-1_f64 * t262);
    let t267 = 0.19751673498613801407e-1_f64 * t237 * t262;
    let t268 = piecewise3(t231, t136, t173);
    let t269 = piecewise3(t234, t136, 0.0_f64);
    let t271 = t268 / 2.0_f64 + t269 / 2.0_f64;
    let t272 = t271 * t271;
    (t265, t267, t271, t272)
}
