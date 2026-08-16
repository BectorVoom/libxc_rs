//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1413/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1413<F: Float>(t10277: F, t344: F, t9288: F, t2979: F, t338: F, t9277: F, t698: F, t986: F, t973: F, t135: F, t3010: F, t241: F, t625: F) -> (F, F, F, F, F, F, F, F) {
    let t10278 = t344 * t10277;
    let t10279 = t10278 * t9288;
    let t10280 = t2979 * t10279;
    let t10283 = t9277 * t338;
    let t10286 = t698 * t986;
    let t10287 = t973 * t10286;
    let t10289 = t135 * t3010;
    let t10290 = t973 * t10289;
    let t10292 = t625 * t241;
    (t10279, t10280, t10283, t10286, t10287, t10289, t10290, t10292)
}
