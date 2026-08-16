//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 948/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk948<F: Float>(t1339: F, t1537: F, t44386: F, t590: F, t44294: F, t41649: F, t41656: F, t41660: F, t44404: F, t475: F, t447: F, t2299: F, t3516: F) -> (F, F, F, F, F, F, F, F) {
    let t46068 = F::cast_from(0.97135412416599232513e1_f64) * t1537 * t1339 * t44386 * t590;
    let t46072 = F::cast_from(0.25561950635947166451e1_f64) * t1537 * t1339 * t44294 * t590;
    let t46073 = F::cast_from(0.15337170381568299871e1_f64) * t41649;
    let t46078 = F::cast_from(0.20705180015117204825e2_f64) * t41656;
    let t46079 = F::cast_from(0.92023022289409799224e1_f64) * t41660;
    let t46080 = t44404 * t475;
    let t46084 = t44404 * t447;
    let t46088 = t2299 * t3516;
    (t46068, t46072, t46073, t46078, t46079, t46080, t46084, t46088)
}
