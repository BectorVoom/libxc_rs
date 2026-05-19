//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1283/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1283<F: Float>(t11922: F, t4906: F, t3115: F, t15957: F, t4910: F, t3117: F, t3075: F, t357: F, t4781: F, t11670: F, t4890: F, t3317: F) -> (F, F, F, F, F) {
    let t16035 = t11922 * t4906;
    let t16037 = F::cast_from(0.28582678745379824648e-3_f64) * t3115 * t16035;
    let t16039 = t15957 * t4910;
    let t16040 = t3117 * t16039;
    let t16043 = t357 * t3075;
    let t16044 = t4781 * t16043;
    let t16045 = t3117 * t16044;
    let t16048 = t11670 * t4890;
    let t16049 = t3317 * t16048;
    (t16037, t16040, t16045, t16048, t16049)
}
