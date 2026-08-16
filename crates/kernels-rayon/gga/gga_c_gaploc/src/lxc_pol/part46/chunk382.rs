//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 382/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk382(t2366: f64, t874: f64, t2365: f64, t1429: f64, t3133: f64, t531: f64, t3137: f64, t3085: f64, t569: f64, t568: f64, t123: f64, t883: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3162 = t2366 * t874;
    let t3163 = t2365 * t3162;
    let t3165 = 0.29792074959875355558e-1_f64 * t1429 * t3163;
    let t3166 = t531 * t3133;
    let t3169 = t531 * t3137;
    let t3172 = t569 * t3085;
    let t3173 = t568 * t3172;
    let t3176 = t874 * t123;
    let t3177 = t3176 * t883;
    (t3162, t3163, t3165, t3166, t3169, t3172, t3173, t3177)
}
