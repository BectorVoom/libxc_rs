//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3725/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3725(t17708: f64, t59948: f64, t17394: f64, t370: f64, t17727: f64, t12916: f64, t21258: f64, t3718: f64, t17753: f64, t21045: f64, t12866: f64, t5401: f64, t58895: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t70639 = t59948 * t17708;
    let t70646 = t17394 * t370;
    let t70647 = t17727 * t70646;
    let t70664 = t3718 * t12916 * t21258;
    let t70667 = t17753 * t12916 * t21045;
    let t70672 = t12866 * t58895 * t5401;
    (t70639, t70646, t70647, t70664, t70667, t70672)
}
