//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2514/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2514(t12378: f64, t300: f64, t13062: f64, t13064: f64, t3172: f64, t1247: f64, t13075: f64, t1209: f64, t13126: f64, t17708: f64, t127: f64, t12988: f64, t12989: f64, t371: f64) -> (f64, f64, f64, f64, f64) {
    let t45319 = t300 * t12378;
    let t45346 = t13062 * t3172 * t13064;
    let t45352 = t1247 * t3172 * t13075;
    let t45371 = t1209 * t13126 * t17708;
    let t45382 = t12988 * t371 * t127 * t12989;
    (t45319, t45346, t45352, t45371, t45382)
}
