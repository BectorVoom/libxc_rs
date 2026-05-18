//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 491/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk491<F: Float>(t2665: F, t446: F, t140: F, t3183: F, t3101: F, t3138: F, t466: F, t429: F, t530: F, t321: F, t457: F, t1167: F, sigma2: F) -> (F, F, F, F, F, F, F) {
    let t3184 = t446 * t2665;
    let t3185 = t3184 * t140;
    let t3186 = t3183 * t3185;
    let t3192 = t3101 * t3185;
    let t3199 = F::new(0.16793568152788065763e-2) * t466 * t3138;
    let t3200 = t530 * t429;
    let t3201 = t321 * t3200;
    let t3203 = F::new(0.19318136643975017455e-1) * t457 * t3201;
    let t3209 = t1167 * sigma2;
    (t3186, t3192, t3199, t3200, t3201, t3203, t3209)
}
