//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 945/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk945(t40775: f64, t1022: f64, t6058: f64, t2508: f64, t28668: f64, t5241: f64, t13176: f64, t2549: f64, t13173: f64, t169: f64, t270: f64, t299: f64, t43081: f64, t43152: f64, t43154: f64, t43156: f64, t43157: f64, t43166: f64, t43168: f64, t43170: f64, t43173: f64, t43175: f64, t43179: f64, t43182: f64, t43185: f64, t43189: f64, t650: f64, t706: f64) -> f64 {
    let t43190 = 0.1922631557535556071e-2_f64 * t40775;
    let t43191 = t6058 * t1022;
    let t43195 = 0.46143157380853345701e0_f64 * t2508 * t43191 * t5241 * t28668;
    let t43196 = t2549 * t13176;
    let t43198 = -t43152 + 0.46143157380853345702e-1_f64 * t43154 + t43156 + t43157 + 0.76905262301422242837e-2_f64 * t270 * t706 * t43081 * t169 * t299 + 0.10254034973522965712e-1_f64 * t650 * t13173 - t43166 - t43168 + 0.15381052460284448567e-1_f64 * t43170 + t43173 + 0.18457262952341338281e0_f64 * t43175 - t43179 + t43182 + t43185 - t43189 - t43190 - t43195 + 0.64087718584518535698e-3_f64 * t43196;
    t43198
}
