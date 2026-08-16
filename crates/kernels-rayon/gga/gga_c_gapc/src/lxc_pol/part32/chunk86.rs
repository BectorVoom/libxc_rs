//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 86/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk86(t218: f64, t220: f64, t36: f64, t217: f64, t43: f64, t40: f64, zeta_threshold: f64) -> (f64, f64) {
    let t219 = t218 <= zeta_threshold;
    let t222 = piecewise3(t219, t36, t220 * t218);
    let t224 = (t217 + t222 - 2.0_f64) * t43;
    let t225 = 2.0_f64 <= zeta_threshold;
    let t227 = piecewise3(t225, t36, 2.0_f64 * t40);
    let t228 = 0.0_f64 <= zeta_threshold;
    let t229 = piecewise3(t228, t36, 0.0_f64);
    let t231 = (t227 + t229 - 2.0_f64) * t43;
    (t224, t231)
}
