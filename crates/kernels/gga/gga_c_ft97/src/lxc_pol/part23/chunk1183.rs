//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1183/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1183<F: Float>(t1495: F, t2681: F, t12001: F, t29162: F, t29194: F, t8392: F, t29199: F, t29204: F, t29209: F, t6353: F, t848: F, t1882: F, t29174: F, t29356: F, t7059: F, t8232: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t114820 = t2681 * t1495;
    let t114827 = t12001 * t29162;
    let t114837 = 2.0 / 27.0 * t8392 * t29194;
    let t114839 = 4.0 / 27.0 * t8392 * t29199;
    let t114841 = 4.0 / 27.0 * t8392 * t29204;
    let t114843 = 4.0 / 81.0 * t8392 * t29209;
    let t114847 = t848 * t6353;
    let t114852 = 2.0 / 9.0 * t1882 * t29174;
    let t114869 = 4.0 / 9.0 * t1882 * t29356;
    let t114886 = t8232 * t7059;
    (t114820, t114827, t114837, t114839, t114841, t114843, t114847, t114852, t114869, t114886)
}
