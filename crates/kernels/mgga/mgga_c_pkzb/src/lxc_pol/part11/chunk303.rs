//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 303/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk303<F: Float>(t1010: F, t1012: F, t1020: F, t1058: F, t135: F, t144: F, t469: F, t494: F, t498: F, t503: F, t547: F, t554: F, t559: F, t560: F, t639: F, t652: F, t972: F) -> (F, F) {
    let t1062 = t1058 * t135 * t144 * t639 + 3.0 * t1020 * t135 * t560 + t1010 + t1012 + t469 + t494 - t498 - t503 + t547 - t554 - t559;
    let t1064 = t652 * t972;
    (t1062, t1064)
}
