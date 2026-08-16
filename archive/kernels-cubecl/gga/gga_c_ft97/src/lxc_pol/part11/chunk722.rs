//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 722/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk722<F: Float>(t255: F, t9802: F, t3892: F, t9797: F, t2347: F, t761: F, t9792: F, t3891: F, t1882: F, t2471: F, t2459: F, t729: F, t773: F) -> (F, F, F, F, F, F, F, F) {
    let t9803 = t9802 * t255;
    let t9804 = t3892 * t9797;
    let t9805 = t9803 * t9804;
    let t9808 = t761 * t2347;
    let t9809 = t9808 * t9792;
    let t9810 = t3891 * t9809;
    let t9813 = t1882 * t2471;
    let t9816 = t729 * t773 * t2459;
    (t9803, t9804, t9805, t9808, t9809, t9810, t9813, t9816)
}
