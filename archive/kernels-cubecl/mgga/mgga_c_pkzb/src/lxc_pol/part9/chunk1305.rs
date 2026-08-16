//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1305/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1305<F: Float>(t1235: F, t5722: F, t46: F, t6515: F, t6524: F, t6456: F, t3206: F, t8446: F, t926: F, t2380: F, t6475: F, t8459: F) -> (F, F, F, F, F) {
    let t22919 = t1235 * t5722;
    let t22920 = t22919 * t46;
    let t22921 = t6515 * t22920;
    let t22924 = t6524 * t22920;
    let t22927 = t6456 * t22920;
    let t22933 = t3206 * t926 * t8446;
    let t22936 = t2380 * t6475 * t8459;
    (t22921, t22924, t22927, t22933, t22936)
}
