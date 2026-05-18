//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 949/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk949<F: Float>(t13203: F, t7129: F, t2508: F, t2963: F, t3276: F, t13097: F, t13173: F, t1897: F, t43202: F, t43203: F, t43204: F, t43205: F, t43206: F, t43207: F, t43208: F, t43209: F, t43212: F, t43216: F, t43220: F, t43222: F, t43224: F, t43231: F, t681: F, t702: F) -> F {
    let t43233 = t7129 * t13203;
    let t43237 = F::new(0.53833683610995569986e-1) * t2508 * t3276 * t2963;
    let t43238 = -t43202 + t43203 - t43204 - t43205 + t43206 + t43207 + t43208 + t43209 + t43212 + t43216 + t43220 + t43222 + F::new(0.64087718584518535698e-3) * t43224 + F::new(0.76905262301422242837e-2) * t681 * t13173 - F::new(0.76905262301422242837e-2) * t1897 * t13097 * t702 + F::new(0.15381052460284448567e-1) * t43231 + F::new(0.30762104920568897134e-1) * t43233 - t43237;
    t43238
}
