//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 158/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk158(t304: f64, t747: f64, t178: f64, t670: f64, t108: f64, t260: f64, t14: f64, t1: f64, t271: f64, t509: f64, t110: f64, t257: f64) -> (f64, f64, f64, f64, f64) {
    let t748 = t304 * t747;
    let t749 = t670 * t178;
    let t752 = t260 * t108;
    let t753 = t752 * t14;
    let t754 = t271 * t1;
    let t755 = t754 * t509;
    let t758 = t110 * t257;
    (t748, t749, t753, t755, t758)
}
