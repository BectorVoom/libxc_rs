//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1290/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1290(t1794: f64, t8190: f64, t73: f64, t30881: f64, t3565: f64, t7635: f64, t6601: f64, t7623: f64, t21188: f64, t26844: f64, t21233: f64, t7624: f64) -> (f64, f64, f64, f64, f64) {
    let t112120 = t8190 * t1794;
    let t112121 = t112120 * t73;
    let t112129 = t30881 * t3565 * t7635;
    let t112179 = t6601 * t7623;
    let t112195 = t26844 * t21188;
    let t112232 = t7624 * t21233;
    (t112121, t112129, t112179, t112195, t112232)
}
