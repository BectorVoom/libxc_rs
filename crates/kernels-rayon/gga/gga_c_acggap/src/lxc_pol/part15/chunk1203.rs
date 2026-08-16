//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1203/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1203(t10017: f64, t2131: f64, t2147: f64, t309: f64, t40861: f64, t7963: f64, t9427: f64, t38778: f64, t7942: f64, t33227: f64, t33778: f64, t38040: f64, t38324: f64, t38329: f64, t38343: f64, t38345: f64, t38348: f64, t39794: f64, t40215: f64, t40868: f64, t556: f64, t7931: f64, t8400: f64, t8791: f64, t9165: f64) -> f64 {
    let t41272 = t2131 * t2147 * t10017 * t309;
    let t41290 = t7963 * t9427 * t40861;
    let t41293 = t7942 * t9427 * t38778;
    let t41295 = t38324 - t33227 + 0.17347256376410398924e1_f64 * t38329 + 0.17347256376410398924e1_f64 * t41272 + 0.34694512752820797848e1_f64 * t7931 * t9427 * t556 * t8791 + 0.26020884564615598386e1_f64 * t8400 * t38040 * t40215 - 0.26020884564615598386e1_f64 * t8400 * t9427 * t39794 - 0.17347256376410398924e1_f64 * t33778 * t9165 - t38343 - t38345 + 0.17347256376410398924e1_f64 * t7931 * t9427 * t40868 - 0.17347256376410398924e1_f64 * t41290 + 0.17347256376410398924e1_f64 * t41293 - t38348;
    t41295
}
