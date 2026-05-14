//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 709/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk709<F: Float>(t262: F, t35847: F, t7782: F, t25809: F, t664: F, t35583: F, t793: F, t35586: F, t797: F, t2123: F, t4616: F, t265: F, t874: F, t876: F, t305: F, t7894: F, t942: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t36277 = t262 * t35847;
    let t36278 = t7782 * t36277;
    let t36280 = t25809 * t664;
    let t36284 = t793 * t35583;
    let t36286 = t797 * t35586;
    let t36288 = t4616 * t2123;
    let t36292 = t874 * t265;
    let t36293 = t36292 * t876;
    let t36294 = t305 * t36293;
    let t36305 = t942 * t7894;
    (t36277, t36278, t36280, t36284, t36286, t36288, t36292, t36293, t36294, t36305)
}
