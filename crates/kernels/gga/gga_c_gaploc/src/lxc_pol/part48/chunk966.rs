//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 966/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk966<F: Float>(t46341: F, t41906: F, t11386: F, t9285: F, t10532: F, t10533: F, t46254: F, t37648: F, t901: F, t2413: F, t37667: F, t11434: F, t2389: F) -> (F, F, F, F, F, F, F) {
    let t46342 = F::new(0.29792074959875355558e-1) * t46341;
    let t46343 = F::new(0.30674340763136599741e1) * t41906;
    let t46345 = F::new(0.35750489951850426669e0) * t9285 * t11386;
    let t46352 = F::new(0.27606906686822939767e2) * t10532 * t10533 * t46254;
    let t46353 = t37648 * t901;
    let t46354 = F::new(0.14896037479937677779e-1) * t46353;
    let t46356 = F::new(0.25025342966295298669e1) * t37667 * t2413;
    let t46360 = t11434 * t2389;
    (t46342, t46343, t46345, t46352, t46354, t46356, t46360)
}
