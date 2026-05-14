//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 921/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk921<F: Float>(t1045: F, t11869: F, t3117: F, t3316: F, t994: F, t4891: F, t11659: F, t4910: F, t1016: F, t697: F, t1011: F, t1010: F, t2270: F, t3241: F, t3244: F, t1058: F, t3197: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11870 = t11869 * t1045;
    let t11871 = t3117 * t11870;
    let t11874 = t994 * t3316;
    let t11875 = t11874 * t4891;
    let t11876 = t11659 * t4910;
    let t11877 = t3117 * t11876;
    let t11880 = t697 * t1016;
    let t11881 = t1011 * t11880;
    let t11883 = t2270 * t1010;
    let t11886 = t3241 * t3244;
    let t11888 = t3197 * t1058;
    (t11871, t11874, t11875, t11877, t11880, t11881, t11883, t11886, t11888)
}
