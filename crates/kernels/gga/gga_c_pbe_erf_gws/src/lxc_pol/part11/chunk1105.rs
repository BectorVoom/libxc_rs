//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1105/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1105<F: Float>(t38681: F, t3134: F, t45088: F, t46382: F, t46098: F, t11514: F, t13347: F, t13431: F, t2343: F, t2345: F, t3235: F, t3247: F, t38683: F, t46013: F, t46023: F, t46078: F, t48985: F, t904: F, t929: F, t933: F) -> (F, F, F, F, F) {
    let t50103 = 35.0 / 72.0 * t38681;
    let t50107 = t45088 * t3134 / 8.0;
    let t50109 = t46382 * t3134 / 8.0;
    let t50110 = 7.0 / 36.0 * t46098;
    let t50111 = t2343 * t2345 * t11514 * t13347 / 64.0 + 7.0 / 192.0 * t46013 - 7.0 / 64.0 * t46023 + 9.0 / 256.0 * t3247 * t3235 * t11514 * t13431 - t929 * t933 * t904 * t48985 / 768.0 - t50103 - 119.0 / 1152.0 * t38683 - 7.0 / 64.0 * t46078 - t50107 - t50109 + t50110;
    (t50103, t50107, t50109, t50110, t50111)
}
