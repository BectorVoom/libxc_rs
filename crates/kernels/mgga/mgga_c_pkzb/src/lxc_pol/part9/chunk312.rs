//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 312/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk312<F: Float>(t1024: F, t581: F, t1010: F, t1012: F, t158: F, t469: F, t494: F, t498: F, t503: F, t547: F, t554: F, t559: F) -> (F, F) {
    let t1025 = t581 * t1024;
    let t1029 = (t469 + t494 - t498 - t503 + t1010 + t547 + t1012 - t554 - t559) * t158;
    (t1025, t1029)
}
