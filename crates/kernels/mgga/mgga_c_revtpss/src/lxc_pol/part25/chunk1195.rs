//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1195/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1195<F: Float>(t281: F, t555: F, t93238: F, t25898: F, t7303: F, t25917: F, t9303: F, t10073: F, t1444: F, t2029: F, t25929: F, t26041: F, t9664: F, t2030: F, t47567: F, t4057: F) -> (F, F, F, F, F, F) {
    let t94849 = t281 * t93238 * t555;
    let t94851 = t94849 * t25898 * t7303;
    let t94854 = 0.26019841438354088051e-2 * t9303 * t25917;
    let t94857 = t10073 * t25929 * t2029 * t1444;
    let t94865 = 0.46263278077393568556e-2 * t26041 * t9664;
    let t94867 = 0.81814717454467823679e-4 * t47567 * t2030;
    let t94868 = t4057 * t1444;
    (t94851, t94854, t94857, t94865, t94867, t94868)
}
