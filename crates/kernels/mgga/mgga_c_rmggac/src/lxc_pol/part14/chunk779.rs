//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 779/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk779<F: Float>(t36271: F, t7788: F, t7834: F, t838: F, t262: F, t35847: F, t7782: F, t25809: F, t664: F, t35583: F, t793: F, t35586: F, t797: F) -> (F, F, F, F, F, F, F) {
    let t36272 = t7788 * t36271;
    let t36274 = t838 * t7834;
    let t36277 = t262 * t35847;
    let t36278 = t7782 * t36277;
    let t36280 = t25809 * t664;
    let t36284 = t793 * t35583;
    let t36286 = t797 * t35586;
    (t36272, t36274, t36277, t36278, t36280, t36284, t36286)
}
