//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1259/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1259<F: Float>(t1006: F, t10335: F, t1139: F, t15711: F, t15721: F, t285: F, t288: F, t2934: F, t15202: F, t3063: F, t2925: F, t2931: F, t15207: F, t852: F, t12630: F, t855: F) -> (F, F, F, F, F, F, F, F) {
    let t43151 = t1006 * t10335;
    let t43179 = t15711 * t1139;
    let t43184 = t285 / t15721 / t288;
    let t43191 = t2934 * t2934;
    let t43192 = 1.0 / t43191;
    let t43200 = t3063 * t15202;
    let t43225 = t2925 * t2931;
    let t43236 = t852 * t15207;
    let t43614 = t12630 * t855;
    (t43151, t43179, t43184, t43192, t43200, t43225, t43236, t43614)
}
