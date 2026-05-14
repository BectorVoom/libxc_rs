//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 649/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk649<F: Float>(t1024: F, t3270: F, t2508: F, t10677: F, t883: F, t2562: F, t943: F, t13202: F, t13204: F, t13208: F, t13211: F, t13214: F, t13215: F, t13216: F, t13219: F, t13220: F) -> (F, F, F) {
    let t13221 = t3270 * t1024;
    let t13223 = 0.76905262301422242837e-2 * t2508 * t13221;
    let t13224 = t883 * t10677;
    let t13225 = t2562 * t13224;
    let t13226 = t943 * t13225;
    let t13228 = -t13202 + 0.30762104920568897134e-1 * t13204 + t13208 + t13211 - t13214 - t13215 + t13216 - t13219 + t13220 + t13223 - 0.64087718584518535698e-3 * t13226;
    (t13221, t13225, t13228)
}
