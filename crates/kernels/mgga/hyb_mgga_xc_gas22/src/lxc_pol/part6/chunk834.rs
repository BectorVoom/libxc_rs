//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 834/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk834<F: Float>(t6358: F, t180: F, t2124: F, t2109: F, t746: F, t172: F, t2018: F, t677: F, t10: F, t2054: F, t6299: F, t138: F, t2022: F) -> (F, F, F, F, F, F, F, F) {
    let t6359 = F::new(1.0) / t6358;
    let t6363 = t180 * t2124;
    let t6383 = F::new(1.0) / t2109 / t746;
    let t6394 = F::new(1.0) / t6358 / t172;
    let t6425 = t677 * t2018;
    let t6429 = t2054 * t10;
    let t6449 = t6299 * t10;
    let t6452 = F::new(1.0) / t138 / t2022;
    (t6359, t6363, t6383, t6394, t6425, t6429, t6449, t6452)
}
