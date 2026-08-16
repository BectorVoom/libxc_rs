//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2809/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2809(t2448: f64, t9292: f64, t11036: f64, t2435: f64, t10994: f64, t2453: f64, t138: f64, t2438: f64, t2771: f64, t2761: f64, t786: f64, t867: f64) -> (f64, f64, f64, f64, f64) {
    let t41004 = t9292 * t2448;
    let t41006 = t2435 * t11036;
    let t41011 = t2453 * t10994;
    let t41014 = t41011 * t138 * t2438 * t2771;
    let t41017 = t786 * t2761 * t867;
    (t41004, t41006, t41011, t41014, t41017)
}
