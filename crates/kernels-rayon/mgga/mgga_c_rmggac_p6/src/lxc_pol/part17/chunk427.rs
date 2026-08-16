//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 427/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk427(t158: f64, t4272: f64, t1079: f64, t166: f64, t4221: f64, t4130: f64, t4133: f64, t4136: f64, t4138: f64, t4142: f64, t4144: f64, t4146: f64, t4149: f64) -> (f64, f64, f64) {
    let t4273 = t158 * t4272;
    let t4275 = 1.0_f64 / t1079 / t166;
    let t4276 = t4221 * t4275;
    let t4287 = -0.25319e1_f64 * t4130 + 0.16879333333333333333e1_f64 * t4133 - 0.19692555555555555555e1_f64 * t4136 - 0.93011851851851851854e0_f64 * t4138 + 0.13651666666666666667e0_f64 * t4142 - 0.27303333333333333333e0_f64 * t4144 - 0.3185388888888888889e0_f64 * t4146 - 0.36514074074074074075e0_f64 * t4149;
    (t4273, t4276, t4287)
}
