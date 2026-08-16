//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2689/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2689(t20020: f64, t3224: f64, t1025: f64, t127: f64, t19768: f64, t371: f64, t225: f64, t64686: f64, t366: f64, t64907: f64, t19773: f64, t3215: f64) -> (f64, f64, f64, f64, f64) {
    let t67493 = t3224 * t20020;
    let t67499 = t1025 * t371 * t127 * t19768;
    let t67501 = t64686 * t225;
    let t67516 = t64907 * t366;
    let t67521 = t19773 * t3215;
    (t67493, t67499, t67501, t67516, t67521)
}
