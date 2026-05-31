//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 893/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk893<F: Float>(t24295: F, t3459: F, t11301: F, t6556: F, t10805: F, t8862: F, t11135: F, t10802: F, t27229: F, t11969: F, t1960: F, t977: F) -> (F, F, F, F, F, F) {
    let t46004 = F::cast_from(4.0_f64) * t24295 * t3459;
    let t46006 = F::cast_from(4.0_f64) * t6556 * t11301;
    let t46008 = F::cast_from(4.0_f64) * t8862 * t10805;
    let t46013 = F::cast_from(4.0_f64) * t8862 * t11135;
    let t46016 = F::cast_from(12.0_f64) * t27229 * t10802;
    let t46019 = F::cast_from(2.0_f64) * t1960 * t11969 * t977;
    (t46004, t46006, t46008, t46013, t46016, t46019)
}
