//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1427/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1427(t1118: f64, t11190: f64, t43970: f64, t3307: f64, t3264: f64, t3313: f64, t3315: f64, t11399: f64, t3403: f64, t11297: f64, t11303: f64, t11310: f64, t11361: f64, t11365: f64, t11430: f64, t11434: f64, t11437: f64, t1155: f64, t1157: f64, t3376: f64, t3377: f64, t3378: f64, t3395: f64, t3401: f64, t3404: f64, t43956: f64, t43958: f64, t43961: f64, t43963: f64, t43966: f64, t43973: f64, t43979: f64, t43984: f64, t43989: f64, t43994: f64) -> (f64, f64, f64, f64) {
    let t44085 = 24.0_f64 * t11190 * t43970 * t1118;
    let t44086 = t3307 * t3307;
    let t44089 = 6.0_f64 * t3264 * t44086 * t1118;
    let t44092 = 0.48245938496077605201e2_f64 * t3313 * t44086 * t3315;
    let t44106 = t11399 * t3403;
    let t44115 = -0.14035736694323150897e2_f64 * t11297 * t11430 + 0.21053605041484726346e2_f64 * t3401 * t3378 * t3395 - 0.46785788981077169656e1_f64 * t3376 * t1157 * t11399 - 0.62337092780453269531e3_f64 * t11365 * t3404 * t3395 + 0.2077903092681775651e3_f64 * t11361 * t11434 + 0.69263436422725855036e2_f64 * t3401 * t44106 * t1155 + 0.61524113149298439947e4_f64 * t11310 * t43984 * t3377 - 24.0_f64 * t11303 * t11437 - t43956 - t43958 - t43961 - t43963 - t43966 + t43973 - t43979 + t43989 - t43994;
    (t44085, t44089, t44092, t44115)
}
