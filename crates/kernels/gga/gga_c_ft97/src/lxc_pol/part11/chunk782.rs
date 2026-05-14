//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 782/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk782<F: Float>(t1608: F, t1630: F, t7998: F, t1632: F, t39: F, t8003: F, t395: F, t45: F, t44: F, t52: F, t54: F, t5588: F, t1527: F, t37315: F, t419: F, t37264: F) -> (F, F, F, F, F, F) {
    let t37668 = t1608 * t7998 * t1630;
    let t37670 = t1632 * t39 * t8003;
    let t37678 = 1.0 / t45 / t395;
    let t37685 = t52 * t54 / t44 / t5588;
    let t37696 = t419 * t1527 * t37315;
    let t37699 = t419 * t1527 * t37264;
    (t37668, t37670, t37678, t37685, t37696, t37699)
}
