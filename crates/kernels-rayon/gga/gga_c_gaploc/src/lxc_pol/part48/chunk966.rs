//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 966/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk966(t46341: f64, t41906: f64, t11386: f64, t9285: f64, t10532: f64, t10533: f64, t46254: f64, t37648: f64, t901: f64, t2413: f64, t37667: f64, t11434: f64, t2389: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t46342 = 0.29792074959875355558e-1_f64 * t46341;
    let t46343 = 0.30674340763136599741e1_f64 * t41906;
    let t46345 = 0.35750489951850426669e0_f64 * t9285 * t11386;
    let t46352 = 0.27606906686822939767e2_f64 * t10532 * t10533 * t46254;
    let t46353 = t37648 * t901;
    let t46354 = 0.14896037479937677779e-1_f64 * t46353;
    let t46356 = 0.25025342966295298669e1_f64 * t37667 * t2413;
    let t46360 = t11434 * t2389;
    (t46342, t46343, t46345, t46352, t46354, t46356, t46360)
}
