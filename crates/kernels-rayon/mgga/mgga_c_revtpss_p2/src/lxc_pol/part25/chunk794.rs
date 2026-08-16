//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 794/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk794(t738: f64, t745: f64, t9385: f64, t1340: f64, t1320: f64, t3853: f64, t123: f64, t147: f64, t9291: f64) -> (f64, f64, f64, f64) {
    let t9387 = t738 * t9385 * t745;
    let t9389 = 0.5848223622634646207e0_f64 * t1340 * t9387;
    let t9391 = 12.0_f64 * t1320 * t3853;
    let t9394 = 0.34450798614814814813e-2_f64 * t123 * t9291 * t147;
    (t9387, t9389, t9391, t9394)
}
