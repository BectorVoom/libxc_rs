//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 900/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk900<F: Float>(t46115: F, t6717: F, t6914: F, t11426: F, t20967: F, t1: F, t37975: F, t1415: F, t1457: F, t2398: F, t10463: F, t10557: F) -> (F, F, F, F, F) {
    let t46118 = F::new(0.12423108009070322895e3) * t6914 * t6717 * t46115;
    let t46119 = t11426 * t20967;
    let t46121 = t37975 * t1;
    let t46125 = F::new(0.42900587942220512003e1) * t1415 * t46121 * t1457 * t2398;
    let t46127 = F::new(0.85801175884441024006e1) * t10557 * t10463;
    (t46118, t46119, t46121, t46125, t46127)
}
