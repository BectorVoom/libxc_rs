//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1205/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1205<F: Float>(t11922: F, t4906: F, t3115: F, t15957: F, t4910: F, t3117: F, t3075: F, t357: F, t4781: F, t11670: F, t4890: F, t3317: F, t3299: F, t4895: F, t4892: F, t140: F, t4886: F) -> (F, F, F, F, F, F, F) {
    let t16035 = t11922 * t4906;
    let t16037 = 0.28582678745379824648e-3 * t3115 * t16035;
    let t16039 = t15957 * t4910;
    let t16040 = t3117 * t16039;
    let t16043 = t357 * t3075;
    let t16044 = t4781 * t16043;
    let t16045 = t3117 * t16044;
    let t16048 = t11670 * t4890;
    let t16049 = t3317 * t16048;
    let t16052 = t3299 * t16048;
    let t16055 = t11922 * t4895;
    let t16057 = 0.57165357490759649296e-3 * t4892 * t16055;
    let t16060 = t140 * t4886;
    (t16037, t16040, t16045, t16049, t16052, t16057, t16060)
}
