//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 230/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk230<F: Float>(t135: F, t626: F, t628: F, t631: F, t636: F, t648: F, t656: F, t661: F) -> (F,) {
    let t664 = -t626 - t628 * t631 / 48.0 - 0.27166129655589868296e-2 * t636 * t648 - t656 - 0.10866451862235947318e-1 * t135 * t661;
    (t664,)
}
