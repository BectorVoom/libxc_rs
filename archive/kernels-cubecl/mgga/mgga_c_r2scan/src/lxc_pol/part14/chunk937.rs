//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 937/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk937<F: Float>(t10781: F, t2127: F, t1060: F, t2185: F, t1058: F, t5103: F, t1543: F, t5095: F, t2252: F, t2201: F, t2207: F, t3328: F, t3336: F) -> (F, F, F, F, F, F, F, F) {
    let t10782 = t10781 * t2127;
    let t10784 = t1060 * t2185;
    let t10786 = t5103 * t1058 * t10784;
    let t10788 = t1060 * t1543;
    let t10790 = t5095 * t1058 * t10788;
    let t10792 = t1060 * t2252;
    let t10794 = t2201 * t1058 * t10792;
    let t10797 = t2207 * t3336 * t3328;
    (t10782, t10784, t10786, t10788, t10790, t10792, t10794, t10797)
}
