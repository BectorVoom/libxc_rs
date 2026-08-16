//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1370/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1370<F: Float>(t96217: F, t26955: F, t26960: F, t92657: F, t93087: F, t96190: F, t96193: F, t96196: F, t96204: F, t96207: F, t96212: F, t96215: F, t96857: F, t97076: F, t97188: F) -> F {
    let t97312 = F::cast_from(0.15476481481481481481e-2_f64) * t96217;
    let t97319 = -F::cast_from(0.23214722222222222222e-2_f64) * t96190 - F::cast_from(0.61905925925925925926e-2_f64) * t96193 - F::cast_from(0.12381185185185185185e-1_f64) * t96196 + F::cast_from(0.20635308641975308642e-2_f64) * t96204 - F::cast_from(0.77382407407407407406e-3_f64) * t96207 - F::cast_from(0.11607361111111111111e-2_f64) * t93087 - F::cast_from(0.34822083333333333332e-2_f64) * t96212 + F::cast_from(0.23214722222222222222e-2_f64) * t96215 + t97312 + F::cast_from(0.15459116753472222222e-4_f64) * t26955 * t97188 - F::cast_from(0.30945286961263020833e-5_f64) * t92657 * t96857 + F::cast_from(0.23168402777777777778e-3_f64) * t26960 * t97076;
    t97319
}
