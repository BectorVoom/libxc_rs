//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 818/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk818<F: Float>(t13176: F, t2549: F, t13173: F, t169: F, t270: F, t299: F, t43081: F, t43152: F, t43154: F, t43156: F, t43157: F, t43166: F, t43168: F, t43170: F, t43173: F, t43175: F, t43179: F, t43182: F, t43185: F, t43189: F, t43190: F, t43195: F, t650: F, t706: F) -> (F,) {
    let t43196 = t2549 * t13176;
    let t43198 = -t43152 + 0.46143157380853345702e-1 * t43154 + t43156 + t43157 + 0.76905262301422242837e-2 * t270 * t706 * t43081 * t169 * t299 + 0.10254034973522965712e-1 * t650 * t13173 - t43166 - t43168 + 0.15381052460284448567e-1 * t43170 + t43173 + 0.18457262952341338281e0 * t43175 - t43179 + t43182 + t43185 - t43189 - t43190 - t43195 + 0.64087718584518535698e-3 * t43196;
    (t43198,)
}
