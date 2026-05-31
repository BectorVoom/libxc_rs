//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1264/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1264<F: Float>(t38681: F, t3134: F, t45088: F, t46382: F, t46098: F, t11514: F, t13347: F, t13431: F, t2343: F, t2345: F, t3235: F, t3247: F, t38683: F, t46013: F, t46023: F, t46078: F, t48985: F, t904: F, t929: F, t933: F) -> (F, F, F, F, F) {
    let t50103 = F::cast_from(35.0_f64) / F::cast_from(72.0_f64) * t38681;
    let t50107 = t45088 * t3134 / F::cast_from(8.0_f64);
    let t50109 = t46382 * t3134 / F::cast_from(8.0_f64);
    let t50110 = F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t46098;
    let t50111 = t2343 * t2345 * t11514 * t13347 / F::cast_from(64.0_f64) + F::cast_from(7.0_f64) / F::cast_from(192.0_f64) * t46013 - F::cast_from(7.0_f64) / F::cast_from(64.0_f64) * t46023 + F::cast_from(9.0_f64) / F::cast_from(256.0_f64) * t3247 * t3235 * t11514 * t13431 - t929 * t933 * t904 * t48985 / F::cast_from(768.0_f64) - t50103 - F::cast_from(119.0_f64) / F::cast_from(1152.0_f64) * t38683 - F::cast_from(7.0_f64) / F::cast_from(64.0_f64) * t46078 - t50107 - t50109 + t50110;
    (t50103, t50107, t50109, t50110, t50111)
}
