//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 927/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk927<F: Float>(t1041: F, t10870: F, t3048: F, t3053: F, t10478: F, t3128: F, t10472: F, t1015: F, t1030: F, t3036: F, t3033: F, t248: F, t3041: F, t3101: F) -> (F, F, F, F, F, F, F) {
    let t10871 = t1041 * t10870;
    let t10873 = t3048 * t3053;
    let t10875 = t3128 * t10478;
    let t10876 = t10472 * t10875;
    let t10882 = t1015 * t10478;
    let t10883 = t10472 * t10882;
    let t10889 = t1030 * t3036;
    let t10890 = t1015 * t10889;
    let t10891 = t3033 * t10890;
    let t10895 = t248 * t3101 * t3041;
    (t10871, t10873, t10876, t10883, t10889, t10891, t10895)
}
