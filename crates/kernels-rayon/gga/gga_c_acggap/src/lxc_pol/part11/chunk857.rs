//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 857/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk857(t30090: f64, t7348: f64, t4680: f64, t7346: f64, t7347: f64, t1165: f64, t16020: f64, t604: f64, t1160: f64, t7432: f64, t7365: f64, t16548: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30091 = t30090 * t7348;
    let t30094 = t7346 * t4680 * t7347;
    let t30099 = t7346 * t1165 * t604 * t16020;
    let t30105 = t1160 * t7432;
    let t30106 = t30105 * t7365;
    let t30110 = t7346 * t1165 * t604 * t16548;
    (t30091, t30094, t30099, t30105, t30106, t30110)
}
