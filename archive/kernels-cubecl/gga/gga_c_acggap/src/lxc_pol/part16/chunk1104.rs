//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1104/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1104<F: Float>(t31146: F, t6375: F, t7815: F, t6379: F, t7450: F, t6383: F, t2030: F, t6300: F, t6304: F, t6309: F, t570: F, t6279: F) -> (F, F, F, F, F, F, F) {
    let t39299 = t31146 * t7815 * t6375;
    let t39302 = t7450 * t7815 * t6379;
    let t39305 = t7450 * t7815 * t6383;
    let t39308 = t2030 * t7815 * t6300;
    let t39311 = t2030 * t7815 * t6304;
    let t39314 = t2030 * t7815 * t6309;
    let t39318 = t570 * t6279;
    (t39299, t39302, t39305, t39308, t39311, t39314, t39318)
}
