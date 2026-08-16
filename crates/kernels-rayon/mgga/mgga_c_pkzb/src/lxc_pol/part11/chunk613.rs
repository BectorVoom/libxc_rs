//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 613/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk613(t12: f64, t24: f64, t124: f64, t3380: f64, t207: f64, t3363: f64, t3366: f64, t652: f64, t333: f64, t3371: f64, t3374: f64, t821: f64, zeta_threshold: f64) -> (f64, f64) {
    let t84 = t12 <= zeta_threshold;
    let t90 = t24 <= zeta_threshold;
    let t3382 = 0.19751673498613801407e-1_f64 * t3380 * t124;
    let t3388 = piecewise3(t84, 0.0_f64, -2.0_f64 / 9.0_f64 * t652 * t3363 + 2.0_f64 / 3.0_f64 * t207 * t3366);
    let t3394 = piecewise3(t90, 0.0_f64, -2.0_f64 / 9.0_f64 * t821 * t3371 + 2.0_f64 / 3.0_f64 * t333 * t3374);
    let t3396 = t3388 / 2.0_f64 + t3394 / 2.0_f64;
    (t3382, t3396)
}
