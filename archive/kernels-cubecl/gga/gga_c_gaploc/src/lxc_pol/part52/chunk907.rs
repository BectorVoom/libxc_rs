//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 907/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk907<F: Float>(t4130: F, t44386: F, t4781: F, t590: F, t44294: F, t493: F, t1441: F, t3529: F, t2482: F, t9272: F, t3377: F, t38185: F) -> (F, F, F, F, F) {
    let t46271 = F::cast_from(0.13803453343411469884e2_f64) * t4781 * t4130 * t44386 * t590;
    let t46272 = t493 * t44294;
    let t46275 = F::cast_from(0.1022478025437886658e1_f64) * t1441 * t46272 * t590;
    let t46283 = F::cast_from(0.15337170381568299871e1_f64) * t4781 * t4130 * t44294 * t590;
    let t46284 = t4130 * t3529;
    let t46286 = t9272 * t46284 * t2482;
    let t46287 = F::cast_from(0.57514388930881124514e0_f64) * t46286;
    let t46289 = F::cast_from(0.10725146985555128001e1_f64) * t38185 * t3377;
    (t46271, t46275, t46283, t46287, t46289)
}
