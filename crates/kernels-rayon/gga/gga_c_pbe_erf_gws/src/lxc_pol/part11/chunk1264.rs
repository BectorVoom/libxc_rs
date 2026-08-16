//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1264/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1264(t38681: f64, t3134: f64, t45088: f64, t46382: f64, t46098: f64, t11514: f64, t13347: f64, t13431: f64, t2343: f64, t2345: f64, t3235: f64, t3247: f64, t38683: f64, t46013: f64, t46023: f64, t46078: f64, t48985: f64, t904: f64, t929: f64, t933: f64) -> (f64, f64, f64, f64, f64) {
    let t50103 = 35.0_f64 / 72.0_f64 * t38681;
    let t50107 = t45088 * t3134 / 8.0_f64;
    let t50109 = t46382 * t3134 / 8.0_f64;
    let t50110 = 7.0_f64 / 36.0_f64 * t46098;
    let t50111 = t2343 * t2345 * t11514 * t13347 / 64.0_f64 + 7.0_f64 / 192.0_f64 * t46013 - 7.0_f64 / 64.0_f64 * t46023 + 9.0_f64 / 256.0_f64 * t3247 * t3235 * t11514 * t13431 - t929 * t933 * t904 * t48985 / 768.0_f64 - t50103 - 119.0_f64 / 1152.0_f64 * t38683 - 7.0_f64 / 64.0_f64 * t46078 - t50107 - t50109 + t50110;
    (t50103, t50107, t50109, t50110, t50111)
}
