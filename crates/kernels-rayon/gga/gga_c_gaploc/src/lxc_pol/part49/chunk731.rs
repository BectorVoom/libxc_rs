//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 731/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk731(t3722: f64, t779: f64, t12214: f64, t2580: f64, t12259: f64, t1901: f64, t12161: f64, t169: f64, t299: f64, t706: f64, t12250: f64, t123: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12291 = t779 * t3722;
    let t12294 = t2580 * t12214;
    let t12297 = t1901 * t12259;
    let t12305 = t12161 * t169 * t299;
    let t12306 = t706 * t12305;
    let t12311 = t12250 * t123;
    (t12291, t12294, t12297, t12305, t12306, t12311)
}
