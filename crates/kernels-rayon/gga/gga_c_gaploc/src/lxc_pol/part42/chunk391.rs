//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 391/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk391(t169: f64, t3529: f64, t172: f64, t452: f64, t203: f64, t3517: f64, t492: f64, t1339: f64, t3516: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3530 = t3529 * t169;
    let t3531 = t3530 * t172;
    let t3532 = t452 * t3531;
    let t3536 = t3517 * t203;
    let t3537 = t492 * t3536;
    let t3541 = t1339 * t3516;
    (t3530, t3531, t3532, t3536, t3537, t3541)
}
