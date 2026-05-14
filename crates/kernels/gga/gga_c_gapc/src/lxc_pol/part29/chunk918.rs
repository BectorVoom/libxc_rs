//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 918/1129 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk918<F: Float>(t2206: F, t932: F, t6851: F, t761: F, t147: F, t19: F, t2254: F, t3296: F, t2153: F, t5692: F, t8: F, t5: F, t17890: F, t277: F, t575: F, t8596: F) -> (F, F, F, F, F, F, F, F) {
    let t24352 = t932 * t2206;
    let t24398 = t761 * t6851;
    let t24499 = t3296 * t2254 * t19 * t147;
    let t24625 = t2153 * t2206;
    let t24759 = 1.0 / t8 / t5692;
    let t24760 = t5 * t24759;
    let t24761 = t277 * t17890;
    let t24906 = t8596 * t575;
    (t24352, t24398, t24499, t24625, t24759, t24760, t24761, t24906)
}
