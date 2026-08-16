//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 172/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk172(t532: f64, t220: f64, t523: f64, t248: f64, param_beta: f64) -> (f64, f64, f64) {
    let t533 = param_beta * t532;
    let t536 = t220 * t523 * t532 + 1.0_f64;
    let t537 = 1.0_f64 / t536;
    let t538 = t248 * t537;
    (t533, t536, t538)
}
