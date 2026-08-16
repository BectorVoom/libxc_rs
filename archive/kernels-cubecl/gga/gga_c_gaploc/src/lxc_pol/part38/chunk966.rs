//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 966/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk966<F: Float>(t46341: F, t41906: F, t11386: F, t9285: F, t10532: F, t10533: F, t46254: F, t37648: F, t901: F, t2413: F, t37667: F, t11434: F, t2389: F) -> (F, F, F, F, F, F, F) {
    let t46342 = F::cast_from(0.29792074959875355558e-1_f64) * t46341;
    let t46343 = F::cast_from(0.30674340763136599741e1_f64) * t41906;
    let t46345 = F::cast_from(0.35750489951850426669e0_f64) * t9285 * t11386;
    let t46352 = F::cast_from(0.27606906686822939767e2_f64) * t10532 * t10533 * t46254;
    let t46353 = t37648 * t901;
    let t46354 = F::cast_from(0.14896037479937677779e-1_f64) * t46353;
    let t46356 = F::cast_from(0.25025342966295298669e1_f64) * t37667 * t2413;
    let t46360 = t11434 * t2389;
    (t46342, t46343, t46345, t46352, t46354, t46356, t46360)
}
