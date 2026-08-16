//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1346/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1346(t15579: f64, t5329: f64, t7773: f64, t15553: f64, t28145: f64, t7788: f64, t26966: f64, t27042: f64, t28132: f64, t28137: f64, t28184: f64, t95736: f64, t95739: f64, t95742: f64, t95745: f64, t95748: f64) -> (f64, f64) {
    let t96831 = t5329 * t7773 * t15579;
    let t96836 = t7788 * t15553 * t28145;
    let t96846 = 0.18534722222222222222e-2_f64 * t26966 * t28132 + 0.37069444444444444444e-2_f64 * t26966 * t28137 + 0.34752604166666666667e-3_f64 * t7788 * t96831 + 0.69644166666666666664e-2_f64 * t95736 + 0.54059606481481481482e-3_f64 * t96836 + 0.18534722222222222222e-2_f64 * t26966 * t28184 + 0.24734586805555555556e-3_f64 * t27042 * t28184 - 0.41270617283950617284e-2_f64 * t95739 + 0.12381185185185185185e-1_f64 * t95742 - 0.10317654320987654321e-1_f64 * t95745 + 0.15476481481481481481e-2_f64 * t95748;
    (t96831, t96846)
}
