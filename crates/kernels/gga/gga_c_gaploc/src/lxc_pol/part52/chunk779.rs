//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 779/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk779<F: Float>(t1339: F, t1537: F, t44294: F, t590: F, t41649: F, t41656: F, t41660: F, t2299: F, t3516: F, t1415: F, t1646: F, t12990: F, t8072: F, t44386: F, t447: F) -> (F, F, F, F, F, F, F) {
    let t46072 = 0.25561950635947166451e1 * t1537 * t1339 * t44294 * t590;
    let t46073 = 0.15337170381568299871e1 * t41649;
    let t46078 = 0.20705180015117204825e2 * t41656;
    let t46079 = 0.92023022289409799224e1 * t41660;
    let t46088 = t2299 * t3516;
    let t46091 = 0.35750489951850426669e0 * t1415 * t46088 * t1646;
    let t46093 = 0.71500979903700853338e0 * t12990 * t8072;
    let t46094 = t44386 * t447;
    (t46072, t46073, t46078, t46079, t46091, t46093, t46094)
}
