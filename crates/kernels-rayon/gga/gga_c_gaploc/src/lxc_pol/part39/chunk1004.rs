//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1004/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1004(t2508: f64, t2580: f64, t43217: f64, t13221: f64, t7129: f64, t2558: f64, t33232: f64, t9647: f64, t13188: f64, t13203: f64, t2963: f64, t3276: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43220 = 0.15381052460284448567e-1_f64 * t2508 * t2580 * t43217;
    let t43222 = 0.76905262301422242837e-2_f64 * t7129 * t13221;
    let t43224 = t9647 * t33232 * t2558;
    let t43231 = t7129 * t13188;
    let t43233 = t7129 * t13203;
    let t43237 = 0.53833683610995569986e-1_f64 * t2508 * t3276 * t2963;
    (t43220, t43222, t43224, t43231, t43233, t43237)
}
