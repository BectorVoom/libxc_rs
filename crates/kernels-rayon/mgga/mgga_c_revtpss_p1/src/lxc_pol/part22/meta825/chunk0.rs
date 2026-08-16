//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2943/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2943(t5552: f64, t588: f64, t5560: f64, t13581: f64, t177: f64, t762: f64, t1317: f64, t13632: f64, t3857: f64, t5569: f64, t512: f64, t749: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t48185 = 32.0_f64 * t5552 * t588;
    let t48212 = 32.0_f64 * t5560 * t588;
    let t48222 = t13581 * t177 * t762;
    let t48225 = t1317 * t13632;
    let t48227 = t3857 * t5569;
    let t48230 = t512 * t13581 * t749;
    (t48185, t48212, t48222, t48225, t48227, t48230)
}
