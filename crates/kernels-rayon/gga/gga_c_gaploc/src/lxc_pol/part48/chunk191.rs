//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 191/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk191(t898: f64, t901: f64, t531: f64, t888: f64, t569: f64, t874: f64, t568: f64, t169: f64, t78: f64) -> (f64, f64, f64, f64, f64) {
    let t902 = t898 * t901;
    let t904 = t531 * t888;
    let t907 = t569 * t874;
    let t908 = t568 * t907;
    let t911 = t78 * t169;
    (t902, t904, t907, t908, t911)
}
