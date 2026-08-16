//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 896/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk896(t2482: f64, t3545: f64, t9263: f64, t1441: f64, t44386: f64, t493: f64, t590: f64, t1339: f64, t1537: f64, t44294: f64, t41649: f64, t41656: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46059 = t9263 * t3545 * t2482;
    let t46060 = 0.38342925953920749676e0_f64 * t46059;
    let t46064 = 0.20449560508757733161e1_f64 * t1441 * t493 * t44386 * t590;
    let t46068 = 0.97135412416599232513e1_f64 * t1537 * t1339 * t44386 * t590;
    let t46072 = 0.25561950635947166451e1_f64 * t1537 * t1339 * t44294 * t590;
    let t46073 = 0.15337170381568299871e1_f64 * t41649;
    let t46078 = 0.20705180015117204825e2_f64 * t41656;
    (t46060, t46064, t46068, t46072, t46073, t46078)
}
