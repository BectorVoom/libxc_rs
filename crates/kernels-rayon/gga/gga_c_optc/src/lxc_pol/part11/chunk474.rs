//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 474/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk474(t2849: f64, t3086: f64, t1113: f64, t2855: f64, t1167: f64, t442: f64) -> (f64, f64, f64) {
    let t3087 = t3086 * t2849;
    let t3092 = t1113 * t2855;
    let t3101 = t1167 * t442;
    (t3087, t3092, t3101)
}
