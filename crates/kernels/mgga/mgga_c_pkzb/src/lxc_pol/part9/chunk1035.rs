//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1035/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1035<F: Float>(t6185: F, t921: F, t466: F, t931: F, t2380: F, t2383: F, t6475: F, t6484: F, t53: F, t6404: F, t179: F, t404: F, t6406: F, t414: F, t6545: F, t6535: F, t914: F) -> (F, F, F, F, F, F, F) {
    let t19166 = t921 * t6185;
    let t19191 = t466 * t931;
    let t19193 = t2380 * t19191 * t2383;
    let t19196 = t2380 * t6475 * t6484;
    let t19203 = t53 * t6404;
    let t19206 = t404 * t179 * t19203 * t6406;
    let t19227 = 1.0 / t6545 / t414;
    let t19264 = t914 * t6535;
    (t19166, t19191, t19193, t19196, t19206, t19227, t19264)
}
