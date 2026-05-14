//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1100/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1100<F: Float>(t7583: F, t92236: F, t2381: F, t26579: F, t7592: F, t209: F, t2415: F, t705: F, t73: F, t9251: F, t26594: F, t7580: F, t92201: F, t92181: F, t91925: F, t91929: F, t91932: F, t91935: F, t91938: F, t91941: F, t91944: F, t91948: F, t92223: F, t92227: F, t92233: F, t92237: F) -> (F, F, F) {
    let t92239 = t92236 * t7583;
    let t92241 = t2381 * t26579;
    let t92242 = t92241 * t7592;
    let t92247 = t209 * t73 * t9251 * t705 * t2415;
    let t92248 = t26594 * t92247;
    let t92250 = t7580 * t92201;
    let t92252 = t26594 * t92181;
    let t92254 = 0.99491666666666666664e-2 * t91925 - 0.99491666666666666664e-2 * t91929 + 0.79593333333333333331e-1 * t91932 + 0.59694999999999999999e-1 * t91935 - 0.29847499999999999999e-1 * t91938 - 0.29847499999999999999e-1 * t91941 + 0.92858888888888888885e-1 * t91944 + 0.59694999999999999999e-1 * t91948 - 0.69505208333333333333e-3 * t92223 + 0.69505208333333333333e-3 * t92227 + 0.49555782539766601562e-5 * t92233 + 0.16217881944444444444e-1 * t92237 + 0.16217881944444444444e-1 * t92239 - 0.97307291666666666666e-2 * t92242 - 0.557015165302734375e-4 * t92248 - 0.2782641015625e-3 * t92250 + 0.55701516530273437501e-4 * t92252;
    (t92241, t92247, t92254)
}
