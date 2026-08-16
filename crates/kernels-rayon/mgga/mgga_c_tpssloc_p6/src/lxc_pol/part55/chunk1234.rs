//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1234/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1234(t1369: f64, t32717: f64, t1831: f64, t31165: f64, t5314: f64, t8466: f64, t22804: f64, t32711: f64, t113966: f64, t113982: f64, t113987: f64, t114000: f64, t120342: f64, t120344: f64, t120348: f64, t120350: f64, t120357: f64, t120363: f64, t120366: f64, t120369: f64, t120372: f64, t120375: f64) -> f64 {
    let t120377 = t32717 * t1369;
    let t120379 = t31165 * t1831;
    let t120381 = t8466 * t5314;
    let t120383 = t22804 * t32711;
    let t120386 = -t120342 / 1536.0_f64 - t120344 / 1536.0_f64 - t120348 / 1536.0_f64 + 7.0_f64 / 2304.0_f64 * t120350 + 5.0_f64 / 384.0_f64 * t120357 + 0.56521858531796547196e-2_f64 * t113966 + 0.13457585364713463618e-3_f64 * t120363 - t113982 + 0.48447307312968469025e-2_f64 * t120366 + 0.48447307312968469025e-2_f64 * t120369 - 0.80745512188280781708e-3_f64 * t120372 + 7.0_f64 / 576.0_f64 * t113987 + 7.0_f64 / 576.0_f64 * t120375 - t120377 / 384.0_f64 - t120379 / 384.0_f64 - t120381 / 384.0_f64 + 0.33913115119077928318e-1_f64 * t120383 + 0.33913115119077928318e-1_f64 * t114000;
    t120386
}
