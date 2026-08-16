//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 962/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk962(t4130: f64, t44386: f64, t4781: f64, t590: f64, t44294: f64, t493: f64, t1441: f64, t3529: f64, t2482: f64, t9272: f64, t3377: f64, t38185: f64) -> (f64, f64, f64, f64, f64) {
    let t46271 = 0.13803453343411469884e2_f64 * t4781 * t4130 * t44386 * t590;
    let t46272 = t493 * t44294;
    let t46275 = 0.1022478025437886658e1_f64 * t1441 * t46272 * t590;
    let t46283 = 0.15337170381568299871e1_f64 * t4781 * t4130 * t44294 * t590;
    let t46284 = t4130 * t3529;
    let t46286 = t9272 * t46284 * t2482;
    let t46287 = 0.57514388930881124514e0_f64 * t46286;
    let t46289 = 0.10725146985555128001e1_f64 * t38185 * t3377;
    (t46271, t46275, t46283, t46287, t46289)
}
