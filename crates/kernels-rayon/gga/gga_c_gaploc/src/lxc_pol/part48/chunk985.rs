//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 985/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk985(t1: f64, t106: f64, t13296: f64, t192: f64, t11386: f64, t2441: f64, t13402: f64, t587: f64, t589: f64, t13403: f64, t1407: f64, t46401: f64, t912: f64) -> (f64, f64, f64, f64, f64) {
    let t46626 = t13296 * t1 * t106 * t192;
    let t46630 = 0.35750489951850426669e0_f64 * t2441 * t11386;
    let t46632 = t587 * t589 * t13402;
    let t46633 = 0.25561950635947166451e0_f64 * t46632;
    let t46634 = t1407 * t13403;
    let t46635 = 0.19171462976960374838e0_f64 * t46634;
    let t46637 = t587 * t912 * t46401;
    (t46626, t46630, t46633, t46635, t46637)
}
