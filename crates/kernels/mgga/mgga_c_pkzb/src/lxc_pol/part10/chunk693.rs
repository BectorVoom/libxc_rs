//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 693/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk693<F: Float>(t3046: F, t834: F, t1174: F, t2215: F, t836: F, t841: F, t1180: F, t218: F, t675: F) -> (F, F, F, F, F) {
    let t3047 = t834 * t3046;
    let t3052 = t2215 * t1174;
    let t3053 = t3052 * t836;
    let t3055 = t841 * t3046;
    let t3059 = t218 * t675 * t1180;
    (t3047, t3052, t3053, t3055, t3059)
}
