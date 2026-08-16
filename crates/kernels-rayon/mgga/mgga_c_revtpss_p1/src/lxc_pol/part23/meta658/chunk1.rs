//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2389/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2389(t10504: f64, t138: f64, t886: f64, t9302: f64, t123: f64, t2465: f64, t9291: f64, t10982: f64, t860: f64, t9646: f64, t10115: f64, t251: f64) -> (f64, f64, f64, f64) {
    let t41098 = t10504 * t138 * t9302 * t886;
    let t41102 = t2465 * t123 * t9291 * t886;
    let t41105 = t9646 * t860 * t10982;
    let t41117 = t10115 * t251;
    (t41098, t41102, t41105, t41117)
}
