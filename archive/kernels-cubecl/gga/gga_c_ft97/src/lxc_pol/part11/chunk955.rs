//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 955/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk955<F: Float>(t37353: F, t37357: F, t39778: F, t89: F, t1546: F, t9030: F, t356: F, t9054: F, t1974: F, t37362: F, t1555: F, t1964: F) -> (F, F, F, F, F) {
    let t39781 = t89 * t37353 * t39778 * t37357;
    let t39784 = t89 * t1546 * t9030;
    let t39788 = t89 * t356 * t9054 * t37357;
    let t39792 = t89 * t356 * t1974 * t37362;
    let t39796 = t89 * t1555 * t1964 * t37362;
    (t39781, t39784, t39788, t39792, t39796)
}
