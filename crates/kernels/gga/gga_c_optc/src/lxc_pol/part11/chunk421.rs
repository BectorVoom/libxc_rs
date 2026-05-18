//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 421/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk421<F: Float>(t349: F, t972: F, t346: F, t2257: F, t2280: F, t92: F, t93: F, t136: F, t3: F, t287: F, t529: F, t362: F) -> (F, F, F, F, F, F, F, F) {
    let t2300 = F::new(1.0) / t972 / t349;
    let t2301 = t346 * t2300;
    let t2305 = F::new(0.96922222222222222222e3) * t2257;
    let t2310 = F::new(0.13111111111111111111e3) * t2280;
    let t2325 = F::new(1.0) / t92 / M_PI * t93;
    let t2335 = t136 * t3;
    let t2350 = t529 * t287;
    let t2351 = t2350 * t362;
    (t2300, t2301, t2305, t2310, t2325, t2335, t2350, t2351)
}
