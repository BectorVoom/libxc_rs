//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1370/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1370(t96217: f64, t26955: f64, t26960: f64, t92657: f64, t93087: f64, t96190: f64, t96193: f64, t96196: f64, t96204: f64, t96207: f64, t96212: f64, t96215: f64, t96857: f64, t97076: f64, t97188: f64) -> f64 {
    let t97312 = 0.15476481481481481481e-2_f64 * t96217;
    let t97319 = -0.23214722222222222222e-2_f64 * t96190 - 0.61905925925925925926e-2_f64 * t96193 - 0.12381185185185185185e-1_f64 * t96196 + 0.20635308641975308642e-2_f64 * t96204 - 0.77382407407407407406e-3_f64 * t96207 - 0.11607361111111111111e-2_f64 * t93087 - 0.34822083333333333332e-2_f64 * t96212 + 0.23214722222222222222e-2_f64 * t96215 + t97312 + 0.15459116753472222222e-4_f64 * t26955 * t97188 - 0.30945286961263020833e-5_f64 * t92657 * t96857 + 0.23168402777777777778e-3_f64 * t26960 * t97076;
    t97319
}
