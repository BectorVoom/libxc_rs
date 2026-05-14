//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 514/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk514<F: Float>(t531: F, t9152: F, t3148: F, t524: F, t3152: F, t189: F, t9127: F, t188: F, t3158: F, t1628: F, t3199: F, t3203: F, t3185: F, t1532: F, t1562: F, t1580: F, t1599: F, t1641: F, t193: F, t3166: F, t3169: F, t3182: F, t3186: F, t4950: F, t557: F, t574: F, t597: F) -> (F, F, F, F, F, F) {
    let t9484 = t531 * t9152;
    let t9487 = t524 * t3148;
    let t9490 = t524 * t3152;
    let t9493 = t189 * t9127;
    let t9494 = t188 * t9493;
    let t9497 = t524 * t3158;
    let t9500 = t1628 * t3199;
    let t9503 = t1628 * t3203;
    let t9506 = t1628 * t3185;
    let t9509 = 0.11502877786176224903e2 * t1580 * t3182 - 0.23005755572352449806e1 * t1641 * t3186 + 0.71500979903700853338e0 * t4950 * t3166 - 0.35750489951850426669e0 * t1599 * t3169 - 0.35750489951850426669e0 * t557 * t9484 + 0.35750489951850426669e0 * t9487 * t193 + 0.35750489951850426669e0 * t9490 * t193 + 0.35750489951850426669e0 * t9494 * t193 - 0.10725146985555128001e1 * t9497 * t1532 - 0.92023022289409799224e1 * t1562 * t9500 + 0.30674340763136599741e1 * t597 * t9503 - 0.30674340763136599741e1 * t574 * t9506;
    (t9487, t9490, t9493, t9494, t9497, t9509)
}
