//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 778/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk778<F: Float>(t37977: F, t44255: F, t549: F, t20367: F, t44387: F, t4820: F, t2375: F, t37575: F, t2482: F, t3545: F, t9263: F, t1441: F, t44386: F, t493: F, t590: F, t1339: F, t1537: F) -> (F, F, F, F, F, F) {
    let t46052 = 0.47667319935800568892e0 * t37977 * t549 * t44255;
    let t46055 = 0.23833659967900284446e0 * t20367 * t4820 * t44387;
    let t46057 = 0.11916829983950142223e0 * t37575 * t2375;
    let t46059 = t9263 * t3545 * t2482;
    let t46060 = 0.38342925953920749676e0 * t46059;
    let t46064 = 0.20449560508757733161e1 * t1441 * t493 * t44386 * t590;
    let t46068 = 0.97135412416599232513e1 * t1537 * t1339 * t44386 * t590;
    (t46052, t46055, t46057, t46060, t46064, t46068)
}
