//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 483/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk483<F: Float>(t2023: F, t401: F, t46: F, t2364: F, t394: F) -> (F, F, F, F) {
    let t2365 = t401 * t2023;
    let t2366 = t2365 * t46;
    let t2367 = t2364 * t2366;
    let t2370 = t394 * t394;
    (t2365, t2366, t2367, t2370)
}
