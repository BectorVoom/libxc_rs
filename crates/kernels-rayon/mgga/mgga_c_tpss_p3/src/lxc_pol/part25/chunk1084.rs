//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1084/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1084(t14947: f64, t967: f64, t11456: f64, t11459: f64, t11462: f64, t14902: f64, t14908: f64, t14913: f64, t14917: f64, t14922: f64, t14925: f64, t14928: f64, t14931: f64, t14935: f64, t14939: f64, t14943: f64, t2731: f64, t2748: f64, t4996: f64, t5005: f64, t8456: f64, t8472: f64, t8577: f64, t8588: f64, t8976: f64, t925: f64) -> f64 {
    let t14948 = t967 * t14947;
    let t14953 = t8976 * t4996 / 576.0_f64 - t14902 / 4608.0_f64 - t8456 / 1296.0_f64 - t8472 / 13824.0_f64 + t967 * t14908 / 768.0_f64 - t967 * t14913 / 1152.0_f64 - t11456 - t11459 + t11462 - t2731 * t14917 / 1536.0_f64 + t8577 * t14922 / 3072.0_f64 - t925 * t14925 / 36.0_f64 + t925 * t14928 / 108.0_f64 + 7.0_f64 / 648.0_f64 * t925 * t14931 - 5.0_f64 / 2304.0_f64 * t967 * t14935 + 5.0_f64 / 6912.0_f64 * t967 * t14939 + 5.0_f64 / 5184.0_f64 * t967 * t14943 + 5.0_f64 / 20736.0_f64 * t14948 + t2748 * t5005 / 432.0_f64 + t8588 / 162.0_f64;
    t14953
}
