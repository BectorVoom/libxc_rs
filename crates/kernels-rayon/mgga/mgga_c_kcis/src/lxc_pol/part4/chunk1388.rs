//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1388/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1388(t15988: f64, t16631: f64, t11882: f64, t11884: f64, t11914: f64, t15971: f64, t15976: f64, t15983: f64, t15986: f64, t16612: f64, t16615: f64, t16620: f64, t16625: f64, t16627: f64, t16629: f64, t16636: f64, t16640: f64, t16644: f64, t16651: f64) -> f64 {
    let t17995 = 0.23214722222222222222e-2_f64 * t15988;
    let t18002 = 0.23214722222222222222e-2_f64 * t16631;
    let t18008 = -0.23214722222222222222e-2_f64 * t15971 - 0.10446625e-1_f64 * t15976 - 0.51588271604938271604e-3_f64 * t11882 + 0.15476481481481481481e-2_f64 * t11884 + 0.10317654320987654321e-2_f64 * t15983 - 0.61905925925925925924e-2_f64 * t15986 - t17995 - 0.17411041666666666666e-2_f64 * t16612 - 0.38691203703703703703e-3_f64 * t16615 + 0.10317654320987654321e-2_f64 * t16620 + 0.34822083333333333332e-2_f64 * t16625 + 0.61905925925925925924e-2_f64 * t16627 - 0.41270617283950617282e-2_f64 * t16629 - t18002 - 0.51588271604938271604e-3_f64 * t16636 - 0.15476481481481481481e-2_f64 * t16640 - 0.15476481481481481481e-2_f64 * t16644 - 0.15476481481481481481e-2_f64 * t11914 + 0.61905925925925925924e-2_f64 * t16651;
    t18008
}
