//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 744/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk744<F: Float>(t3401: F, t50: F, t581: F, t3396: F, t1034: F) -> (F, F, F, F, F) {
    let t3402 = t50 * t3401;
    let t3403 = t581 * t3402;
    let t3406 = t50 * t3396;
    let t3407 = t581 * t3406;
    let t3410 = t1034 * t1034;
    (t3402, t3403, t3406, t3407, t3410)
}
