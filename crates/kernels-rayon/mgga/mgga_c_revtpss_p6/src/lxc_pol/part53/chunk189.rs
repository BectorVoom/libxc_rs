//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 189/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk189(t252: f64, t786: f64, t257: f64, t72: f64, t686: f64) -> (f64, f64, f64) {
    let t787 = t786 * t252;
    let t788 = t257 * t72;
    let t789 = t788 * t686;
    (t787, t788, t789)
}
