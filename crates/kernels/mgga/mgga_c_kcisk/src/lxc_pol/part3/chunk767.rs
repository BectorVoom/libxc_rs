//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 767/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk767<F: Float>(t11786: F, t5321: F, t10479: F, t7311: F, t5278: F, t5295: F, t11154: F, t4971: F, t735: F, t734: F, t1871: F, t5272: F) -> (F, F, F, F, F) {
    let t11787 = t5321 * t11786;
    let t11789 = t7311 * t10479;
    let t11790 = t5321 * t11789;
    let t11792 = t5278 * t5295;
    let t11794 = t4971 * t11154;
    let t11795 = t735 * t11794;
    let t11796 = t734 * t11795;
    let t11798 = t5272 * t1871;
    (t11787, t11790, t11792, t11796, t11798)
}
