//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 642/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk642<F: Float>(t4963: F, t1385: F, t378: F, t735: F, t1527: F, t1751: F, t1411: F, t1398: F, t468: F, t1510: F, t410: F, t4911: F, t89: F, t36: F, t409: F, t732: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4964 = 0.32530743900905219526e-1 * t4963;
    let t4965 = t378 * t1385;
    let t4966 = t735 * t4965;
    let t4967 = 0.48159733137676571078e0 * t4966;
    let t4968 = t1751 * t1527;
    let t4970 = t378 * t1411;
    let t4971 = t735 * t4970;
    let t4972 = 0.16265371950452609763e-1 * t4971;
    let t4973 = t1398 * t468;
    let t4974 = t735 * t4973;
    let t4975 = 0.21687162600603479684e-1 * t4974;
    let t4978 = t410 * t1510;
    let t4979 = 12.0 * t4978;
    let t4980 = t4911 * t89;
    let t4981 = 24.0 * t4980;
    let t4982 = t36 * t409;
    let t4983 = t4982 * t89;
    let t4987 = t732 * t1385;
    (t4964, t4967, t4968, t4972, t4975, t4979, t4981, t4982, t4983, t4987)
}
