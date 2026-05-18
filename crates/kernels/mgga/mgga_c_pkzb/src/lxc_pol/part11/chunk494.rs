//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 494/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk494<F: Float>(t42: F, t448: F, t459: F, t987: F, t1424: F, t973: F, t440: F, t15: F, t8: F, t1429: F, t1435: F, t983: F) -> (F, F, F, F, F, F) {
    let t2481 = t448 * t42;
    let t2484 = t987 * t459;
    let t2489 = t1424 * t973;
    let t2490 = t2489 * t440;
    let t2493 = t15 * t8;
    let t2494 = t2493 * t1429;
    let t2499 = t1435 * t983;
    (t2481, t2484, t2489, t2490, t2494, t2499)
}
