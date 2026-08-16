//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 943/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk943(t2728: f64, t3638: f64, t5559: f64, t1382: f64, t2902: f64, t3418: f64, t13567: f64, t747: f64, t24295: f64, t3459: f64, t11301: f64, t6556: f64) -> (f64, f64, f64, f64, f64) {
    let t45997 = 6.0_f64 * t5559 * t3638 * t2728;
    let t46000 = 4.0_f64 * t1382 * t2902 * t3418;
    let t46001 = t13567 * t747;
    let t46004 = 4.0_f64 * t24295 * t3459;
    let t46006 = 4.0_f64 * t6556 * t11301;
    (t45997, t46000, t46001, t46004, t46006)
}
