//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 943/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk943<F: Float>(t2728: F, t3638: F, t5559: F, t1382: F, t2902: F, t3418: F, t13567: F, t747: F, t24295: F, t3459: F, t11301: F, t6556: F) -> (F, F, F, F, F) {
    let t45997 = F::cast_from(6.0_f64) * t5559 * t3638 * t2728;
    let t46000 = F::cast_from(4.0_f64) * t1382 * t2902 * t3418;
    let t46001 = t13567 * t747;
    let t46004 = F::cast_from(4.0_f64) * t24295 * t3459;
    let t46006 = F::cast_from(4.0_f64) * t6556 * t11301;
    (t45997, t46000, t46001, t46004, t46006)
}
