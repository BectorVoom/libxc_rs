//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1255/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1255<F: Float>(t43191: F, t15202: F, t3063: F, t2925: F, t2931: F, t15207: F, t852: F, t12630: F, t855: F, t135: F, t15206: F, t60: F, t3375: F, t15698: F, t3374: F, t1097: F, t15704: F) -> (F, F, F, F, F, F, F, F, F) {
    let t43192 = 1.0 / t43191;
    let t43200 = t3063 * t15202;
    let t43225 = t2925 * t2931;
    let t43236 = t852 * t15207;
    let t43614 = t12630 * t855;
    let t43655 = t60 / t15206 / t135;
    let t43669 = t3375 * t3375;
    let t43670 = 1.0 / t43669;
    let t43674 = t3374 * t15698;
    let t43680 = t1097 * t15704;
    (t43192, t43200, t43225, t43236, t43614, t43655, t43670, t43674, t43680)
}
