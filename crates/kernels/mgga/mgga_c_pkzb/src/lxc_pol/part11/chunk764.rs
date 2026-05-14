//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 764/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk764<F: Float>(t7930: F, t1003: F, t6097: F, t2179: F, t8: F, t1180: F, t1878: F, t218: F) -> (F, F, F, F) {
    let t7931 = 0.59793333333333333334e0 * t7930;
    let t7932 = t6097 * t1003;
    let t7935 = t2179 * t8;
    let t7950 = t218 * t1878 * t1180;
    (t7931, t7932, t7935, t7950)
}
