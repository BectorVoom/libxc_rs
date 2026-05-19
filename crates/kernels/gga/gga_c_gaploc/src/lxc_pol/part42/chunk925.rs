//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 925/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk925<F: Float>(t10348: F, t3566: F, t11386: F, t2441: F, t13402: F, t587: F, t589: F, t13403: F, t1407: F, t46401: F, t912: F, t11167: F, t2464: F, t2465: F) -> (F, F, F, F, F, F) {
    let t46614 = F::cast_from(0.16683561977530199113e1_f64) * t3566 * t10348;
    let t46630 = F::cast_from(0.35750489951850426669e0_f64) * t2441 * t11386;
    let t46632 = t587 * t589 * t13402;
    let t46633 = F::cast_from(0.25561950635947166451e0_f64) * t46632;
    let t46634 = t1407 * t13403;
    let t46635 = F::cast_from(0.19171462976960374838e0_f64) * t46634;
    let t46637 = t587 * t912 * t46401;
    let t46638 = F::cast_from(0.19171462976960374838e0_f64) * t46637;
    let t46641 = t587 * t2464 * t2465 * t11167;
    (t46614, t46630, t46633, t46635, t46638, t46641)
}
