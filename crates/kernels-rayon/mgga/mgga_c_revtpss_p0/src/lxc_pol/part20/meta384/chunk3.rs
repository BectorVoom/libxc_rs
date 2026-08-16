//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1405/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1405(t123: f64, t2465: f64, t886: f64, t9291: f64, t10982: f64, t860: f64, t9646: f64, t2434: f64, t2828: f64, t10115: f64, t251: f64, t887: f64) -> (f64, f64, f64, f64) {
    let t41102 = t2465 * t123 * t9291 * t886;
    let t41105 = t9646 * t860 * t10982;
    let t41115 = t2465 * t123 * t2434 * t2828;
    let t41117 = t10115 * t251;
    let t41118 = t41117 * t887;
    (t41102, t41105, t41115, t41118)
}
