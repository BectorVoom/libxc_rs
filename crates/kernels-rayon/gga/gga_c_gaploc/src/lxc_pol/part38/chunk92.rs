//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 92/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk92(t121: f64, t423: f64, t158: f64, t169: f64, t172: f64, t110: f64, t9: f64, t19: f64, t3: f64, t108: f64, t14: f64, t23: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t424 = t423 * t121;
    let t425 = t424 * t158;
    let t426 = t169 * t172;
    let t427 = t9 * t110;
    let t432 = t19 / t3;
    let t433 = t108 * t108;
    let t434 = t433 * t433;
    let t435 = t434 * t108;
    let t436 = t432 * t435;
    let t437 = t23 * t14;
    (t424, t425, t426, t427, t436, t437)
}
