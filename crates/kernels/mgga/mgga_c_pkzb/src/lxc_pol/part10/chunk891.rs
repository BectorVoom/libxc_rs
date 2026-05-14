//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 891/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk891<F: Float>(t2383: F, t6475: F, t2380: F, t2185: F, t394: F, t2099: F, t2372: F, t2367: F, t2397: F, t2395: F, t912: F, t5717: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6476 = t6475 * t2383;
    let t6477 = t2380 * t6476;
    let t6483 = t394 * t2185;
    let t6488 = t2099 * t2372;
    let t6489 = t2367 * t6488;
    let t6491 = t2099 * t2397;
    let t6492 = t2395 * t6491;
    let t6512 = t912 * t912;
    let t6513 = 1.0 / t6512;
    let t6514 = t5717 * t6513;
    (t6476, t6477, t6483, t6488, t6489, t6491, t6492, t6512, t6513, t6514)
}
