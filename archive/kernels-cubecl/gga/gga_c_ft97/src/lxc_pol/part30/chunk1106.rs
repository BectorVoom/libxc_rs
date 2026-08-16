//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1106/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1106<F: Float>(t193: F, t24964: F, t7021: F, t89: F, t28719: F, t6222: F, t33966: F, t4129: F, t35863: F, t684: F, t24976: F, t6317: F) -> (F, F, F, F, F) {
    let t152834 = t89 * t193 * t24964 * t7021;
    let t152838 = t89 * t193 * t6222 * t28719;
    let t152842 = t89 * t193 * t33966 * t4129;
    let t152844 = t35863 * t684;
    let t152846 = t6317 * t24976 * t152844;
    (t152834, t152838, t152842, t152844, t152846)
}
