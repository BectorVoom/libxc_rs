//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 721/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk721(t13188: f64, t2508: f64, t3251: f64, t9014: f64, t10628: f64, t5539: f64, t9647: f64, t12605: f64, t12609: f64, t13168: f64, t13173: f64, t13177: f64, t13180: f64, t13184: f64, t13187: f64, t270: f64) -> (f64, f64, f64) {
    let t13189 = t2508 * t13188;
    let t13191 = t9014 * t3251;
    let t13193 = 0.92286314761706691403e-1_f64 * t2508 * t13191;
    let t13194 = t5539 * t10628;
    let t13195 = t9647 * t13194;
    let t13196 = 0.12817543716903707139e-2_f64 * t13195;
    let t13197 = 0.1922631557535556071e-2_f64 * t12605;
    let t13198 = 0.1281754371690370714e-2_f64 * t12609;
    let t13199 = -0.76905262301422242837e-2_f64 * t270 * t13168 + 0.76905262301422242837e-2_f64 * t270 * t13173 + 0.64087718584518535698e-3_f64 * t13177 - 0.46143157380853345702e-1_f64 * t13180 + t13184 - t13187 + 0.15381052460284448567e-1_f64 * t13189 + t13193 + t13196 - t13197 + t13198;
    (t13191, t13194, t13199)
}
