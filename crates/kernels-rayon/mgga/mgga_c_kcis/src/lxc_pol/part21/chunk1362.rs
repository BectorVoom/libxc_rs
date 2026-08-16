//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1362/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1362(t1267: f64, t15534: f64, t167: f64, t28110: f64, t15239: f64, t26960: f64, t26961: f64, t67957: f64, t7788: f64, t8087: f64, t92613: f64, t92896: f64, t92898: f64, t92908: f64, t92921: f64, t92929: f64, t92931: f64, t95989: f64, t95992: f64, t96714: f64) -> (f64, f64) {
    let t97141 = t15534 * t28110 * t167 * t1267;
    let t97147 = -0.69505208333333333334e-3_f64 * t7788 * t96714 - 0.15445601851851851852e-3_f64 * t92896 - 0.15445601851851851852e-3_f64 * t92898 - 0.41270617283950617284e-2_f64 * t92908 - 0.61782407407407407408e-3_f64 * t92921 - 0.15476481481481481481e-2_f64 * t92929 + 0.33980324074074074074e-2_f64 * t92613 * t8087 + 0.46336805555555555556e-3_f64 * t26960 * t15239 * t26961 * t67957 + 0.46336805555555555556e-3_f64 * t26960 * t97141 - 0.30918233506944444444e-4_f64 * t92931 + 0.61905925925925925926e-2_f64 * t95989 + 0.46429444444444444443e-2_f64 * t95992;
    (t97141, t97147)
}
