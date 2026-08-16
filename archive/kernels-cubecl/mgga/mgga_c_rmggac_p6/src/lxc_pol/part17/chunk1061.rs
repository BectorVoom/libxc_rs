//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1061/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1061<F: Float>(t40694: F, t9222: F, t2019: F, t2020: F, t9746: F, t2010: F, t2012: F, t6492: F, t10002: F, t1364: F, t2024: F, t235: F, t2604: F, t36528: F, t41654: F, t41657: F, t41668: F, t46316: F, t46846: F, t47405: F, t47408: F, t47410: F, t47414: F, t47417: F, t47421: F, t515: F, t6304: F, t687: F) -> F {
    let t47429 = t9222 * t40694;
    let t47432 = t2019 * t2020 * t9746;
    let t47435 = t2010 * t2012 * t6492;
    let t47437 = -F::cast_from(0.42564599893297839398e-5_f64) * t47405 + F::cast_from(0.59590439850616975158e-4_f64) * t41654 - t41657 - F::cast_from(0.5987120850931904282e-1_f64) * t47408 + F::cast_from(0.2993560425465952141e-1_f64) * t47410 + t36528 - F::cast_from(0.19957069503106347607e-1_f64) * t6304 * t687 - F::cast_from(0.27933018679976707106e-4_f64) * t47414 - F::cast_from(0.5987120850931904282e-1_f64) * t47417 + t41668 - F::cast_from(0.59871208509319042821e-1_f64) * t2604 * t10002 - F::cast_from(0.2363e1_f64) * t47421 + F::cast_from(0.47896966807455234256e0_f64) * t1364 * t2024 * t46846 - F::cast_from(0.19957069503106347607e-1_f64) * t235 * t515 * t46316 + F::cast_from(0.1064114997332445985e-4_f64) * t47429 - F::cast_from(0.15243824895787514157e-3_f64) * t47432 - F::cast_from(0.36021158228745895953e-3_f64) * t47435;
    t47437
}
