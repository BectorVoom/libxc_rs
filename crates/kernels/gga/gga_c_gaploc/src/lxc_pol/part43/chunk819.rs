//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 819/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk819<F: Float>(t13732: F, t6313: F, t105: F, t169: F, t172: F, t452: F, t46952: F, t1063: F, t38267: F, t894: F, t13725: F, t484: F, t197: F, t3689: F, t161: F, t1358: F, t2339: F) -> (F, F, F, F, F, F) {
    let t46980 = t6313 * t13732;
    let t46991 = 0.28455006635676149599e-1 * t105 * t452 * t46952 * t169 * t172;
    let t47001 = t1063 * t894 * t38267;
    let t47003 = t484 * t13725;
    let t47008 = t197 * t3689;
    let t47009 = t47008 * t161;
    let t47011 = t1358 * t47009 * t2339;
    (t46980, t46991, t47001, t47003, t47008, t47011)
}
