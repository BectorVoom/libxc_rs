//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 474/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk474<F: Float>(t1375: F, t3575: F, t1376: F, t960: F, t3583: F, t457: F, t1384: F, t965: F, t1383: F, t1186: F, t167: F, t3532: F, t3579: F, t1398: F, t158: F, t165: F, t173: F, t3278: F, t3819: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3870 = t1375 * t3575;
    let t3873 = t960 * t1376;
    let t3875 = t1375 * t3583;
    let t3878 = t457 * t3575;
    let t3881 = t965 * t1384;
    let t3883 = t1383 * t3583;
    let t3886 = t1186 * t3575;
    let t3891 = t167 * t3532;
    let t3894 = t1383 * t3579;
    let t3897 = t1398 * t3579;
    let t3900 = t1375 * t3579;
    let t3903 = -0.672175e-5 * t173 * t3870 + 0.9368e-2 * t3873 - 0.3513e-2 * t158 * t3875 + 0.1171e-2 * t158 * t3878 - 0.26416666666666666666e-2 * t3881 + 0.7925e-3 * t165 * t3883 - 0.52833333333333333333e-3 * t165 * t3886 - 0.23911438650126355246e-1 * t3819 * t3278 + 0.15538616723388920628e-3 * t3891 * t3278 - 0.1585e-2 * t165 * t3894 - 0.10082625e-4 * t173 * t3897 + 0.7026e-2 * t158 * t3900;
    (t3870, t3875, t3878, t3883, t3886, t3891, t3894, t3897, t3900, t3903)
}
