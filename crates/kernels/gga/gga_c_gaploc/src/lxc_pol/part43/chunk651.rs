//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 651/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk651<F: Float>(t12605: F, t12609: F, t13184: F, t13187: F, t13193: F, t13195: F, t13919: F, t13922: F, t13925: F, t13935: F, t13938: F, t13202: F, t13208: F, t13211: F, t13214: F, t13215: F, t13216: F, t13219: F, t13220: F, t13223: F, t13944: F, t13947: F) -> (F, F) {
    let t14489 = 0.15381052460284448567e-1 * t13919 - 0.64087718584518535698e-3 * t13935 + 0.30762104920568897134e-1 * t13922 + t13184 - t13187 + 0.64087718584518535698e-3 * t13938 - 0.46143157380853345702e-1 * t13925 + t13193 + 0.1281754371690370714e-2 * t13195 - 0.19226315575355560709e-2 * t12605 + 0.12817543716903707139e-2 * t12609;
    let t14490 = -t13202 + t13208 + t13211 - t13214 - t13215 + t13216 - t13219 + t13220 + t13223 + t13944 - t13947;
    (t14489, t14490)
}
