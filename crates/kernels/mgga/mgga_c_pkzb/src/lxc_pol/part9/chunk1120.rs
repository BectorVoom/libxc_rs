//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1120/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1120<F: Float>(t179: F, t2185: F, t404: F, t6380: F, t2405: F, t6106: F, t2410: F, t344: F, t148: F, t931: F, t824: F, t2411: F, t465: F) -> (F, F, F, F, F, F) {
    let t19128 = t404 * t179 * t6380 * t2185;
    let t19133 = t404 * t179 * t2405 * t6106;
    let t19140 = F::new(1.0) / t2410 / t344;
    let t19150 = t148 * t931;
    let t19153 = t404 * t179 * t19150 * t824;
    let t19155 = t465 * t2411;
    (t19128, t19133, t19140, t19150, t19153, t19155)
}
