//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2676/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2676<F: Float>(t13920: F, t555: F, t14122: F, t14171: F, t46433: F, t46536: F, t46540: F, t46542: F, t46561: F, t46563: F, t46568: F, t46570: F, t5675: F, t5735: F, t5745: F, t5755: F, t820: F, t9840: F, t9912: F) -> (F, F) {
    let t49213 = t555 * t13920;
    let t49233 = F::cast_from(0.39512695097613069591e1_f64) * t5745 * t49213 * t5675 - F::cast_from(0.21951497276451705329e-1_f64) * t46536 + F::cast_from(0.54878743191129263322e-2_f64) * t46540 - F::cast_from(0.21951497276451705329e-1_f64) * t46542 + F::cast_from(0.39512695097613069591e1_f64) * t820 * t14171 * t9912 + F::cast_from(0.39512695097613069591e1_f64) * t5745 * t14122 * t9840 - F::cast_from(0.19756347548806534796e1_f64) * t5755 * t5735 * t46433 + F::cast_from(0.16463622957338778996e-1_f64) * t46561 - F::cast_from(0.43902994552903410657e-1_f64) * t46563 + F::cast_from(0.16463622957338778996e-1_f64) * t46568 + F::cast_from(0.51220160311720645767e-1_f64) * t46570;
    (t49213, t49233)
}
