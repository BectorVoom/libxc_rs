//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 883/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk883<F: Float>(t2383: F, t6475: F, t2380: F, t2382: F, t2442: F, t2381: F, t2185: F, t394: F, t944: F, t2099: F, t2372: F, t2367: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6476 = t6475 * t2383;
    let t6477 = t2380 * t6476;
    let t6479 = t2442 * t2382;
    let t6480 = t2381 * t6479;
    let t6483 = t394 * t2185;
    let t6484 = t944 * t6483;
    let t6485 = t2381 * t6484;
    let t6488 = t2099 * t2372;
    let t6489 = t2367 * t6488;
    (t6476, t6477, t6479, t6480, t6483, t6484, t6485, t6488, t6489)
}
