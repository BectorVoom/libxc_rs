//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1402/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1402<F: Float>(t58265: F, t58308: F, t58310: F, t58315: F, t58572: F, t58591: F, t58629: F, t58651: F, t58788: F, t58797: F, t58800: F, t1102: F, t15582: F, t45062: F) -> (F, F) {
    let t59083 = -t58265 + t58308 - t58310 - t58315 - t58572 + t58591 - t58629 - t58651 - t58788 + t58797 - t58800;
    let t59086 = F::new(0.61523382126046769581e4) * t1102 * t15582 * t45062;
    (t59083, t59086)
}
