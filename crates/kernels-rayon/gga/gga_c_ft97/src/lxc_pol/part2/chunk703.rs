//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 703/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk703(t1636: f64, t89: f64, t943: f64, t3057: f64, t401: f64, t1595: f64, t930: f64, t7914: f64, t3056: f64, t383: f64, t35: f64, t1594: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11076 = t89 * t1636 * t943;
    let t11080 = t3057 * t401;
    let t11084 = t930 * t1595;
    let t11085 = t7914 * t11084;
    let t11088 = t3056 * t383;
    let t11089 = t11088 * t35;
    let t11090 = t1594 * t11089;
    (t11076, t11080, t11084, t11085, t11088, t11089, t11090)
}
