//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 477/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk477<F: Float>(t1144: F, t231: F, t5354: F, t68: F, t181: F, t4342: F, t1131: F, t577: F, t155: F, t4345: F, t1532: F, t446: F, t1415: F, t381: F, t4352: F, t183: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5412 = t1144 * t231;
    let t5415 = t5354 * t68;
    let t5417 = 0.19751673498613801407e-1 * t5415 * t181;
    let t5418 = 0.23392894490538584828e1 * t4342;
    let t5419 = t577 * t1131;
    let t5420 = t155 * t5419;
    let t5421 = 0.18311447306006545054e-3 * t4345;
    let t5422 = t1532 * t446;
    let t5425 = t381 * t1415;
    let t5426 = 8.0 * t5425;
    let t5427 = 0.4883052614935078681e-3 * t4352;
    let t5428 = t5415 * t183;
    (t5412, t5417, t5418, t5420, t5421, t5422, t5426, t5427, t5428)
}
