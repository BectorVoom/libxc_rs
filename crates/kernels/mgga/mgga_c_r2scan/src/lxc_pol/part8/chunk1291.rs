//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1291/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1291<F: Float>(t3198: F, t6512: F, t3056: F, t560: F, t6085: F, t6086: F, t2191: F, t9464: F, t2219: F, t9469: F, t7601: F, t8153: F, t6407: F, t9377: F, t9319: F, t2139: F, t22709: F, t8752: F) -> (F, F, F, F, F, F, F, F) {
    let t30458 = t6512 * t3198;
    let t30468 = t3056 * t560;
    let t30470 = t6085 * t6086 * t30468;
    let t30473 = t9464 * t2191;
    let t30475 = t9469 * t2219;
    let t30496 = t7601 * t8153;
    let t30498 = t6407 * t9377;
    let t30500 = t6407 * t9319;
    let t30535 = t2139 * t22709 * t8752;
    (t30458, t30470, t30473, t30475, t30496, t30498, t30500, t30535)
}
