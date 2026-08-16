//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 101/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk101(t246: f64, t219: f64, t73: f64, t220: f64, t229: f64, param_beta: f64) -> (f64, f64, f64, f64) {
    let t247 = param_beta * t246;
    let t248 = t219 * t73;
    let t251 = t220 * t229 * t246 + 1.0_f64;
    let t252 = 1.0_f64 / t251;
    let t253 = t248 * t252;
    (t247, t248, t251, t253)
}
