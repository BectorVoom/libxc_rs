//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1207/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1207<F: Float>(t1125: F, t17955: F, t757: F, t2096: F, t7581: F, t735: F, t7628: F, t154: F, t2739: F, t276: F, t5688: F, t17938: F, t18290: F, t751: F, t7804: F, t16129: F) -> (F, F, F, F, F, F, F) {
    let t21933 = t757 * t17955 * t1125;
    let t21935 = t2096 * t7581;
    let t21946 = t735 * t7628;
    let t21950 = t276 * t154 * t5688 * t2739;
    let t22007 = t17938 * t18290;
    let t22063 = t751 * t7804;
    let t22148 = 24.0 * t16129;
    (t21933, t21935, t21946, t21950, t22007, t22063, t22148)
}
