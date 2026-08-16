//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 397/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk397<F: Float>(t3236: F, t1229: F, t154: F, t636: F, t2296: F, t1097: F, t419: F, t409: F, t407: F, t410: F, t281: F, t2820: F, t415: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3237 = F::cast_from(0.23744444444444444444e-1_f64) * t3236;
    let t3240 = t154 * t1229;
    let t3241 = t636 * t636;
    let t3242 = F::cast_from(1.0_f64) / t3241;
    let t3247 = F::cast_from(1.0_f64) / t2296;
    let t3262 = t1097 * t419;
    let t3263 = F::cast_from(1.0_f64) / t3262;
    let t3264 = t409 * t3263;
    let t3270 = F::cast_from(1.0_f64) / t410 / t407;
    let t3274 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t3236;
    let t3282 = F::cast_from(0.39862222222222222223e0_f64) * t3236;
    let t3287 = F::cast_from(1.0_f64)/F::sqrt(t407);
    let t3293 = t281 * t2820 * t415;
    (t3237, t3240, t3242, t3247, t3264, t3270, t3274, t3282, t3287, t3293)
}
