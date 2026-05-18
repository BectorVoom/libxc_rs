//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 896/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk896<F: Float>(t2482: F, t3545: F, t9263: F, t1441: F, t44386: F, t493: F, t590: F, t1339: F, t1537: F, t44294: F, t41649: F, t41656: F) -> (F, F, F, F, F, F) {
    let t46059 = t9263 * t3545 * t2482;
    let t46060 = F::new(0.38342925953920749676e0) * t46059;
    let t46064 = F::new(0.20449560508757733161e1) * t1441 * t493 * t44386 * t590;
    let t46068 = F::new(0.97135412416599232513e1) * t1537 * t1339 * t44386 * t590;
    let t46072 = F::new(0.25561950635947166451e1) * t1537 * t1339 * t44294 * t590;
    let t46073 = F::new(0.15337170381568299871e1) * t41649;
    let t46078 = F::new(0.20705180015117204825e2) * t41656;
    (t46060, t46064, t46068, t46072, t46073, t46078)
}
