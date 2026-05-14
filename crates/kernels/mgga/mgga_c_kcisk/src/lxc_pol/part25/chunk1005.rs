//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1005/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1005<F: Float>(t2586: F, t5316: F, t5315: F, t5331: F, t741: F, t5295: F, t7337: F, t1757: F, t6702: F, t7311: F, t5321: F, t5310: F, t7437: F, t2567: F, t5298: F, t734: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t17799 = t2586 * t5316;
    let t17800 = t5315 * t17799;
    let t17802 = t2586 * t5331;
    let t17803 = t741 * t17802;
    let t17805 = t7337 * t5295;
    let t17807 = t6702 * t1757;
    let t17808 = t7311 * t17807;
    let t17809 = t5321 * t17808;
    let t17811 = t5310 * t7437;
    let t17813 = t2567 * t5298;
    let t17814 = t734 * t17813;
    (t17799, t17800, t17802, t17803, t17805, t17807, t17808, t17809, t17811, t17814)
}
