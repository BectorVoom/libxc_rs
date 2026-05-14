//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 576/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk576<F: Float>(t134: F, t3698: F, t137: F, t200: F, t203: F) -> (F, F, F, F) {
    let t3699 = t134 * t134;
    let t3700 = t3698 * t3699;
    let t3702 = t137 * t200 * t203;
    let t3703 = t3700 * t3702;
    (t3699, t3700, t3702, t3703)
}
