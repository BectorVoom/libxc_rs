//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 222/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk222(t123: f64, t173: f64, t186: f64, t651: f64, t654: f64, t679: f64, t699: f64, t706: f64, t714: f64, t721: f64) -> f64 {
    let t724 = 0.53237641966666666666e-3_f64 * t123 * t651 * t173 + 1.0_f64 * t699 * t706 - t654 - t679 + 0.18311447306006545054e-3_f64 * t123 * t651 * t186 + 0.5848223622634646207e0_f64 * t714 * t721;
    t724
}
