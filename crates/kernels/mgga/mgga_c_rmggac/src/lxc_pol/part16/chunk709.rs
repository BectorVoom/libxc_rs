//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 709/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk709<F: Float>(t504: F, t7191: F, t1179: F, t1966: F, t1968: F, t483: F, t1338: F, t2039: F, t303: F, t638: F, t132: F, t26078: F, t36: F, t4787: F, t71: F, t2184: F, t465: F) -> (F, F, F, F, F) {
    let t36639 = t504 * t7191;
    let t36662 = t1966 * t1179 * t483 * t1968;
    let t36674 = t638 * t2039 * t303 * t1338;
    let t36700 = t638 * t36 * t26078 * t71 * t132 * t4787;
    let t36733 = t465 * t2184;
    (t36639, t36662, t36674, t36700, t36733)
}
