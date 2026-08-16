//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 757/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk757(t35900: f64, t987: f64, t1352: f64, t3517: f64, t2754: f64, t986: f64, t6508: f64, t11279: f64, t161: f64, t11218: f64, t158: f64, t20368: f64, t35845: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35901 = t987 * t35900;
    let t35908 = t3517 * t1352;
    let t35912 = t986 * t2754;
    let t35913 = t6508 * t35912;
    let t35918 = t11279 * t161;
    let t35951 = t158 * t11218;
    let t35959 = t20368 * t35845;
    (t35901, t35908, t35913, t35918, t35951, t35959)
}
