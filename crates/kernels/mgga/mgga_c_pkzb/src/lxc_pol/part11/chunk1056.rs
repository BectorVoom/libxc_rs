//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1056/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1056<F: Float>(t179: F, t19155: F, t3757: F, t404: F, t10055: F, t2380: F, t6475: F, t2402: F, t3860: F, t2407: F, t10258: F, t8406: F, t10266: F, t2099: F, t3235: F, t10262: F) -> (F, F, F, F, F, F, F) {
    let t28316 = t404 * t179 * t19155 * t3757;
    let t28324 = t2380 * t6475 * t10055;
    let t28333 = t3860 * t2402;
    let t28335 = t3860 * t2407;
    let t28345 = t10258 * t8406;
    let t28353 = t3235 * t2099 * t10266;
    let t28364 = t3235 * t2099 * t10262;
    (t28316, t28324, t28333, t28335, t28345, t28353, t28364)
}
