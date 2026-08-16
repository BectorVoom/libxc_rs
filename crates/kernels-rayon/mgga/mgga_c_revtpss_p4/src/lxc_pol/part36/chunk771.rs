//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 771/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk771(t572: f64, t7953: f64, t1469: f64, t1479: f64, t61: f64, t6971: f64, t7571: f64, t72: f64, t1927: f64) -> (f64, f64, f64, f64) {
    let t7955 = 3.0_f64 * t572 * t7953;
    let t8142 = -8.0_f64 / 3.0_f64 * t1479 * t61 - 5.0_f64 / 6.0_f64 * t7571 * t1469 + t6971;
    let t8143 = t8142 * t72;
    let t8144 = t8143 * t1927;
    (t7955, t8142, t8143, t8144)
}
