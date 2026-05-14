//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 923/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk923<F: Float>(t17402: F, t17423: F, t17399: F, t45: F, t7147: F, t11153: F, t638: F, t10831: F, t9: F, t662: F, t5005: F, t963: F, t5002: F, t7219: F, t10409: F, t6663: F) -> (F, F, F, F, F, F, F, F, F) {
    let t17594 = 0.13418888888888888889e0 * t17402;
    let t17602 = 0.44152e0 * t17423;
    let t17635 = 0.23744444444444444444e-1 * t17399;
    let t17656 = t45 * t7147;
    let t17697 = t638 * t11153;
    let t17716 = t9 * t10831;
    let t17717 = t17716 * t662;
    let t17721 = t963 * t5005;
    let t17722 = t17721 * t662;
    let t17726 = t7219 * t5002;
    let t17739 = t10409 * t6663;
    (t17594, t17602, t17635, t17656, t17697, t17717, t17722, t17726, t17739)
}
