//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1022/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1022(t2103: f64, t41048: f64, t41032: f64, t36166: f64, t36157: f64, t36158: f64, t36160: f64, t36168: f64, t36174: f64, t41336: f64, t41338: f64, t41341: f64, t41342: f64, t41344: f64, t41348: f64, t41349: f64, t41351: f64) -> f64 {
    let t41353 = t2103 * t41048;
    let t41355 = t2103 * t41032;
    let t41358 = 0.19513579069703984327e0_f64 * t36166;
    let t41360 = -0.11974241701863808564e0_f64 * t41336 - 0.15965655602485078085e0_f64 * t41338 - t41341 + 0.3193131120497015617e0_f64 * t41342 - 0.11974241701863808564e0_f64 * t41344 + t36157 + 0.2660942600414179681e-1_f64 * t36158 - t41348 + 0.9072038638458063915e-4_f64 * t41349 + 0.34093327067806677162e-2_f64 * t41351 - 0.45457769423742236216e-2_f64 * t41353 - 0.44447596769881297634e-1_f64 * t41355 - 0.39914139006212695215e-1_f64 * t36160 - t41358 + 0.29270368604555976491e0_f64 * t36168 - t36174;
    t41360
}
