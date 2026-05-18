//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 897/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk897<F: Float>(t201: F, t9742: F, t199: F, t3719: F, t967: F, t1162: F, t3298: F, t3147: F, t3157: F, t1217: F, t8028: F, t3153: F) -> (F, F, F, F, F, F) {
    let t9743 = t201 * t9742;
    let t9744 = t199 * t9743;
    let t9746 = t3719 * t967;
    let t9748 = t1162 * t3298;
    let t9751 = F::new(0.11696447245269292414e1) * t3147 * t3157;
    let t9753 = F::new(0.11696447245269292414e1) * t8028 * t1217;
    let t9755 = F::new(0.23392894490538584828e1) * t3147 * t3153;
    (t9744, t9746, t9748, t9751, t9753, t9755)
}
