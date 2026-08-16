//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1061/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1061(t40694: f64, t9222: f64, t2019: f64, t2020: f64, t9746: f64, t2010: f64, t2012: f64, t6492: f64, t10002: f64, t1364: f64, t2024: f64, t235: f64, t2604: f64, t36528: f64, t41654: f64, t41657: f64, t41668: f64, t46316: f64, t46846: f64, t47405: f64, t47408: f64, t47410: f64, t47414: f64, t47417: f64, t47421: f64, t515: f64, t6304: f64, t687: f64) -> f64 {
    let t47429 = t9222 * t40694;
    let t47432 = t2019 * t2020 * t9746;
    let t47435 = t2010 * t2012 * t6492;
    let t47437 = -0.42564599893297839398e-5_f64 * t47405 + 0.59590439850616975158e-4_f64 * t41654 - t41657 - 0.5987120850931904282e-1_f64 * t47408 + 0.2993560425465952141e-1_f64 * t47410 + t36528 - 0.19957069503106347607e-1_f64 * t6304 * t687 - 0.27933018679976707106e-4_f64 * t47414 - 0.5987120850931904282e-1_f64 * t47417 + t41668 - 0.59871208509319042821e-1_f64 * t2604 * t10002 - 0.2363e1_f64 * t47421 + 0.47896966807455234256e0_f64 * t1364 * t2024 * t46846 - 0.19957069503106347607e-1_f64 * t235 * t515 * t46316 + 0.1064114997332445985e-4_f64 * t47429 - 0.15243824895787514157e-3_f64 * t47432 - 0.36021158228745895953e-3_f64 * t47435;
    t47437
}
