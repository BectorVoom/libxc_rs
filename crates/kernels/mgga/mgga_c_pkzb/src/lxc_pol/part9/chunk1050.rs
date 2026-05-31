//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1050/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1050<F: Float>(t174: F, t46: F, t1769: F, t5307: F, t5292: F, t2590: F, t5278: F, t5257: F, t5275: F, t1702: F, t5270: F, t5224: F, t575: F) -> (F, F, F, F, F, F, F) {
    let t16322 = t174 * t174;
    let t16323 = F::cast_from(1.0_f64) / t16322;
    let t16324 = t16323 * t46;
    let t16335 = t1769 * t5307;
    let t16341 = t1769 * t5292;
    let t16343 = t2590 * t5278;
    let t16356 = t5257 * t5275;
    let t16363 = t1702 * t5270;
    let t16369 = t575 * t5224;
    (t16324, t16335, t16341, t16343, t16356, t16363, t16369)
}
