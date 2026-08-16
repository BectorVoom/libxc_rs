//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3025/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3025(t14728: f64, t9775: f64, t1549: f64, t40861: f64, t14779: f64, t40721: f64, t221: f64, t40724: f64, t10777: f64, t14787: f64, t14495: f64, t40834: f64, t826: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t50939 = t9775 * t14728;
    let t50941 = t40861 * t1549;
    let t50943 = t40721 * t14779;
    let t50945 = t40724 * t221;
    let t50947 = t10777 * t50945 * t14787;
    let t50954 = t40834 * t826 * t14495;
    (t50939, t50941, t50943, t50945, t50947, t50954)
}
