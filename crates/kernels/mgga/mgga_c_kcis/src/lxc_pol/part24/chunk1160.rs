//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1160/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1160<F: Float>(t26594: F, t92247: F, t7580: F, t92201: F, t92181: F, t91925: F, t91929: F, t91932: F, t91935: F, t91938: F, t91941: F, t91944: F, t91948: F, t92223: F, t92227: F, t92233: F, t92237: F, t92239: F, t92242: F) -> F {
    let t92248 = t26594 * t92247;
    let t92250 = t7580 * t92201;
    let t92252 = t26594 * t92181;
    let t92254 = F::cast_from(0.99491666666666666664e-2_f64) * t91925 - F::cast_from(0.99491666666666666664e-2_f64) * t91929 + F::cast_from(0.79593333333333333331e-1_f64) * t91932 + F::cast_from(0.59694999999999999999e-1_f64) * t91935 - F::cast_from(0.29847499999999999999e-1_f64) * t91938 - F::cast_from(0.29847499999999999999e-1_f64) * t91941 + F::cast_from(0.92858888888888888885e-1_f64) * t91944 + F::cast_from(0.59694999999999999999e-1_f64) * t91948 - F::cast_from(0.69505208333333333333e-3_f64) * t92223 + F::cast_from(0.69505208333333333333e-3_f64) * t92227 + F::cast_from(0.49555782539766601562e-5_f64) * t92233 + F::cast_from(0.16217881944444444444e-1_f64) * t92237 + F::cast_from(0.16217881944444444444e-1_f64) * t92239 - F::cast_from(0.97307291666666666666e-2_f64) * t92242 - F::cast_from(0.557015165302734375e-4_f64) * t92248 - F::cast_from(0.2782641015625e-3_f64) * t92250 + F::cast_from(0.55701516530273437501e-4_f64) * t92252;
    t92254
}
