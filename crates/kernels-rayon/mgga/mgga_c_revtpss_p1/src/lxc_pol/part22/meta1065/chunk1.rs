//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3815/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3815(t198: f64, t3828: f64, t39483: f64, t39520: f64, t39528: f64, t73345: f64, t73350: f64, t73353: f64, t73354: f64, t73355: f64, t73356: f64, t73357: f64, t73358: f64) -> f64 {
    let t73359 = 12.0_f64 * t198 * t3828 * t73345 - t39483 + t39520 - t39528 + t73350 - t73353 - t73354 + t73355 + t73356 + t73357 - t73358;
    t73359
}
