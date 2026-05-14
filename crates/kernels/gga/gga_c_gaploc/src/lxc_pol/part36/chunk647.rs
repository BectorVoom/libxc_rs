//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 647/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk647<F: Float>(t13185: F, t2508: F, t3433: F, t954: F, t3251: F, t9014: F, t10628: F, t5539: F, t9647: F, t12605: F, t12609: F, t13168: F, t13173: F, t13177: F, t13180: F, t13184: F, t270: F) -> (F, F, F, F) {
    let t13187 = 0.53833683610995569986e-1 * t2508 * t13185;
    let t13188 = t954 * t3433;
    let t13189 = t2508 * t13188;
    let t13191 = t9014 * t3251;
    let t13193 = 0.92286314761706691403e-1 * t2508 * t13191;
    let t13194 = t5539 * t10628;
    let t13195 = t9647 * t13194;
    let t13196 = 0.12817543716903707139e-2 * t13195;
    let t13197 = 0.1922631557535556071e-2 * t12605;
    let t13198 = 0.1281754371690370714e-2 * t12609;
    let t13199 = -0.76905262301422242837e-2 * t270 * t13168 + 0.76905262301422242837e-2 * t270 * t13173 + 0.64087718584518535698e-3 * t13177 - 0.46143157380853345702e-1 * t13180 + t13184 - t13187 + 0.15381052460284448567e-1 * t13189 + t13193 + t13196 - t13197 + t13198;
    (t13188, t13191, t13194, t13199)
}
