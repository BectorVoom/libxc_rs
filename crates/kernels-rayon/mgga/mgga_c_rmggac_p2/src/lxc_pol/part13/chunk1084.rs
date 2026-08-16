//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1084/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1084(t41338: f64, t41347: f64, t41355: f64, t36158: f64, t36160: f64, t36166: f64, t36168: f64, t37536: f64, t37544: f64, t41336: f64, t41340: f64, t41342: f64, t41344: f64, t41349: f64, t41351: f64, t41353: f64) -> f64 {
    let t43606 = 0.31931311204970156172e0_f64 * t41338;
    let t43611 = 0.9676841214355268176e-3_f64 * t41347;
    let t43615 = 0.88895193539762595267e-1_f64 * t41355;
    let t43619 = -0.23948483403727617128e0_f64 * t41336 - t43606 - 0.21241846568096930142e-1_f64 * t41340 + 0.63862622409940312342e0_f64 * t41342 - 0.23948483403727617128e0_f64 * t41344 + t37536 + 0.53218852008283593618e-1_f64 * t36158 - t43611 + 0.1814407727691612783e-3_f64 * t41349 + 0.68186654135613354324e-2_f64 * t41351 - 0.90915538847484472432e-2_f64 * t41353 - t43615 - 0.79828278012425390427e-1_f64 * t36160 - 0.39027158139407968654e0_f64 * t36166 + 0.5854073720911195298e0_f64 * t36168 - t37544;
    t43619
}
