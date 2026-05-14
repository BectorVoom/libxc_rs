//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 776/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk776<F: Float>(t3684: F, t7822: F, t2728: F, t3638: F, t5559: F, t24295: F, t3459: F, t11301: F, t6556: F, t10805: F, t8862: F, t11135: F, t10802: F, t27229: F, t11969: F, t1960: F, t977: F) -> (F, F, F, F, F, F, F, F) {
    let t45993 = t7822 * t3684;
    let t45997 = 6.0 * t5559 * t3638 * t2728;
    let t46004 = 4.0 * t24295 * t3459;
    let t46006 = 4.0 * t6556 * t11301;
    let t46008 = 4.0 * t8862 * t10805;
    let t46013 = 4.0 * t8862 * t11135;
    let t46016 = 12.0 * t27229 * t10802;
    let t46019 = 2.0 * t1960 * t11969 * t977;
    (t45993, t45997, t46004, t46006, t46008, t46013, t46016, t46019)
}
