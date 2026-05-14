//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1263/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1263<F: Float>(t111019: F, t111389: F, t111392: F, t111396: F, t111398: F, t111400: F, t111403: F, t111405: F, t111407: F, t111409: F, t111412: F, t31970: F, t9320: F, t9307: F, t2676: F, t43225: F) -> (F, F, F, F) {
    let t111414 = 0.120625e-1 * t111389 - 0.69841875000000000003e-2 * t111392 + 0.40208333333333333335e-2 * t111396 + 0.31250000000000000001e-1 * t111398 + 0.10416666666666666667e-1 * t111400 + 0.99491666666666666664e-2 * t111019 - 0.36187500000000000001e-1 * t111403 + 0.62500000000000000002e-1 * t111405 + 0.31250000000000000001e-1 * t111407 + 0.31250000000000000001e-1 * t111409 + 0.120625e-1 * t111412;
    let t111416 = t31970 * t9320;
    let t111418 = t31970 * t9307;
    let t111421 = t43225 * t2676 * t9307;
    (t111414, t111416, t111418, t111421)
}
