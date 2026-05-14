//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 652/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk652<F: Float>(t1024: F, t3732: F, t14364: F, t169: F, t299: F, t706: F, t13195: F, t13201: F, t13537: F, t13544: F, t13547: F, t13550: F, t13554: F, t13558: F, t14428: F, t2508: F, t270: F) -> (F, F, F, F) {
    let t14431 = t3732 * t1024;
    let t14435 = t14364 * t169 * t299;
    let t14436 = t706 * t14435;
    let t14439 = t13537 + t13544 - t13547 + t13550 - t13554 + t13558 + 0.25635087433807414279e-2 * t13195 - 0.38452631150711121419e-2 * t13201 - 0.46143157380853345702e-1 * t2508 * t14428 + 0.15381052460284448567e-1 * t2508 * t14431 + 0.76905262301422242837e-2 * t270 * t14436;
    (t14431, t14435, t14436, t14439)
}
