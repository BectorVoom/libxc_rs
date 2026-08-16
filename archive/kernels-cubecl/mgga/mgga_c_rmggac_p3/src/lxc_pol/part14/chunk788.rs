//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 788/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk788<F: Float>(t36012: F, t903: F, t1179: F, t1966: F, t1968: F, t483: F, t7367: F, t1249: F, t880: F, t1338: F, t2039: F, t303: F, t638: F) -> (F, F, F, F, F) {
    let t36646 = t903 * t36012;
    let t36662 = t1966 * t1179 * t483 * t1968;
    let t36663 = t36662 * t7367;
    let t36669 = t1249 * t880;
    let t36674 = t638 * t2039 * t303 * t1338;
    (t36646, t36662, t36663, t36669, t36674)
}
