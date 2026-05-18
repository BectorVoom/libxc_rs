//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1425/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1425<F: Float>(t224: F, t36263: F, t36300: F, t36458: F, t37309: F, t12050: F, t12045: F, t12063: F, t12041: F, t12296: F, t12053: F, t12043: F) -> (F, F, F, F, F, F, F, F) {
    let t37312 = t224 * (t36263 + t36300 + t36458 + t37309);
    let t37317 = F::new(4.0) * t12050;
    let t37318 = F::new(2.0) * t12045;
    let t37319 = F::new(4.0) * t12063;
    let t37320 = F::new(2.0) * t12041;
    let t37322 = F::new(2.0) * t12296;
    let t37323 = F::new(12.0) * t12053;
    let t37324 = F::new(4.0) * t12043;
    (t37312, t37317, t37318, t37319, t37320, t37322, t37323, t37324)
}
