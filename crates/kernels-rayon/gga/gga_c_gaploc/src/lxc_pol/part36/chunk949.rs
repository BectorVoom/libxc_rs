//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 949/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk949(t13203: f64, t7129: f64, t2508: f64, t2963: f64, t3276: f64, t13097: f64, t13173: f64, t1897: f64, t43202: f64, t43203: f64, t43204: f64, t43205: f64, t43206: f64, t43207: f64, t43208: f64, t43209: f64, t43212: f64, t43216: f64, t43220: f64, t43222: f64, t43224: f64, t43231: f64, t681: f64, t702: f64) -> f64 {
    let t43233 = t7129 * t13203;
    let t43237 = 0.53833683610995569986e-1_f64 * t2508 * t3276 * t2963;
    let t43238 = -t43202 + t43203 - t43204 - t43205 + t43206 + t43207 + t43208 + t43209 + t43212 + t43216 + t43220 + t43222 + 0.64087718584518535698e-3_f64 * t43224 + 0.76905262301422242837e-2_f64 * t681 * t13173 - 0.76905262301422242837e-2_f64 * t1897 * t13097 * t702 + 0.15381052460284448567e-1_f64 * t43231 + 0.30762104920568897134e-1_f64 * t43233 - t43237;
    t43238
}
