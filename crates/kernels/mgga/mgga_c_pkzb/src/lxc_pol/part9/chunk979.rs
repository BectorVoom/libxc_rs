//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 979/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk979<F: Float>(t1528: F, t204: F, t5063: F, t148: F, t1598: F, t1602: F, t1527: F, t5008: F, t174: F, t46: F, t1769: F, t5307: F, t5292: F, t2590: F, t5278: F, t5257: F, t5275: F) -> (F, F, F, F, F, F, F, F) {
    let t16283 = 0.14246666666666666666e0 * t204 * t5063 * t1528;
    let t16287 = 0.22911460125803964958e1 * t204 * t148 * t1598 * t1602;
    let t16290 = 0.57895126195293126241e3 * t5008 * t1602 * t1527;
    let t16322 = t174 * t174;
    let t16323 = 1.0 / t16322;
    let t16324 = t16323 * t46;
    let t16335 = t1769 * t5307;
    let t16341 = t1769 * t5292;
    let t16343 = t2590 * t5278;
    let t16356 = t5257 * t5275;
    (t16283, t16287, t16290, t16324, t16335, t16341, t16343, t16356)
}
