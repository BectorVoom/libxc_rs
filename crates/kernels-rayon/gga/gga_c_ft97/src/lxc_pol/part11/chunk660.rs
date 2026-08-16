//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 660/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk660(t9071: f64, t2086: f64, t2120: f64, t590: f64, t91: f64, t151: f64, t3051: f64, t1771: f64, t588: f64, t2102: f64, t9041: f64, t9045: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9166 = 28.0_f64 / 27.0_f64 * t9071;
    let t9170 = t91 * t2086 * t590 * t2120;
    let t9178 = 28.0_f64 / 27.0_f64 * t3051 * t151;
    let t9179 = t1771 * t588;
    let t9181 = t2102 * t9041;
    let t9183 = t2102 * t9045;
    (t9166, t9170, t9178, t9179, t9181, t9183)
}
