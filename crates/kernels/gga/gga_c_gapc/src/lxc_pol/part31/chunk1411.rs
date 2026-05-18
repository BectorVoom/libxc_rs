//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1411/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1411<F: Float>(t35358: F, t35361: F, t12191: F, t883: F, t972: F, t12050: F, t12045: F, t12053: F, t12043: F, t12047: F, t12046: F, t12059: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t37294 = F::new(0.14759453667534722223e-5) * t35358;
    let t37295 = F::new(0.20220636637604418766e-5) * t35361;
    let t37306 = t12191 * t883;
    let t37308 = F::new(2.0) * t37306 * t972;
    let t37317 = F::new(4.0) * t12050;
    let t37318 = F::new(2.0) * t12045;
    let t37323 = F::new(12.0) * t12053;
    let t37324 = F::new(4.0) * t12043;
    let t37325 = F::new(4.0) * t12047;
    let t37327 = F::new(2.0) * t12046;
    let t37328 = F::new(4.0) * t12059;
    (t37294, t37295, t37308, t37317, t37318, t37323, t37324, t37325, t37327, t37328)
}
