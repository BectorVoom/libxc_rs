//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1216/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1216(t26594: f64, t92247: f64, t7580: f64, t92201: f64, t92181: f64, t91925: f64, t91929: f64, t91932: f64, t91935: f64, t91938: f64, t91941: f64, t91944: f64, t91948: f64, t92223: f64, t92227: f64, t92233: f64, t92237: f64, t92239: f64, t92242: f64) -> f64 {
    let t92248 = t26594 * t92247;
    let t92250 = t7580 * t92201;
    let t92252 = t26594 * t92181;
    let t92254 = 0.99491666666666666664e-2_f64 * t91925 - 0.99491666666666666664e-2_f64 * t91929 + 0.79593333333333333331e-1_f64 * t91932 + 0.59694999999999999999e-1_f64 * t91935 - 0.29847499999999999999e-1_f64 * t91938 - 0.29847499999999999999e-1_f64 * t91941 + 0.92858888888888888885e-1_f64 * t91944 + 0.59694999999999999999e-1_f64 * t91948 - 0.69505208333333333333e-3_f64 * t92223 + 0.69505208333333333333e-3_f64 * t92227 + 0.49555782539766601562e-5_f64 * t92233 + 0.16217881944444444444e-1_f64 * t92237 + 0.16217881944444444444e-1_f64 * t92239 - 0.97307291666666666666e-2_f64 * t92242 - 0.557015165302734375e-4_f64 * t92248 - 0.2782641015625e-3_f64 * t92250 + 0.55701516530273437501e-4_f64 * t92252;
    t92254
}
