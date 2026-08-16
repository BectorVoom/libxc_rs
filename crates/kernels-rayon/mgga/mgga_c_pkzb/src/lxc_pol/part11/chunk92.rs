//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 92/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk92(t212: f64, t228: f64, t86: f64, t95: f64, t98: f64, zeta_threshold: f64) -> (f64, f64) {
    let t230 = 0.621814e-1_f64 * t212 * t228;
    let t231 = 2.0_f64 <= zeta_threshold;
    let t233 = piecewise3(t231, t86, 2.0_f64 * t95);
    let t234 = 0.0_f64 <= zeta_threshold;
    let t235 = piecewise3(t234, t86, 0.0_f64);
    let t237 = (t233 + t235 - 2.0_f64) * t98;
    (t230, t237)
}
