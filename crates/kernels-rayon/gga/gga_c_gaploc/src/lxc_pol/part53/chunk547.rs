//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 547/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk547(t3259: f64, t747: f64, t3263: f64, t841: f64, t2728: f64, t977: f64, t3322: f64, t2617: f64, t948: f64, t7803: f64, t3251: f64, t590: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9767 = t3259 * t747;
    let t9777 = t3263 * t841;
    let t9780 = t977 * t2728;
    let t9784 = t3322 * t841;
    let t9787 = t948 * t2617;
    let t9788 = t7803 * t9787;
    let t9789 = 0.38342925953920749676e0_f64 * t9788;
    let t9790 = t3251 * t590;
    (t9767, t9777, t9780, t9784, t9788, t9789, t9790)
}
