//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 893/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk893(t24295: f64, t3459: f64, t11301: f64, t6556: f64, t10805: f64, t8862: f64, t11135: f64, t10802: f64, t27229: f64, t11969: f64, t1960: f64, t977: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46004 = 4.0_f64 * t24295 * t3459;
    let t46006 = 4.0_f64 * t6556 * t11301;
    let t46008 = 4.0_f64 * t8862 * t10805;
    let t46013 = 4.0_f64 * t8862 * t11135;
    let t46016 = 12.0_f64 * t27229 * t10802;
    let t46019 = 2.0_f64 * t1960 * t11969 * t977;
    (t46004, t46006, t46008, t46013, t46016, t46019)
}
