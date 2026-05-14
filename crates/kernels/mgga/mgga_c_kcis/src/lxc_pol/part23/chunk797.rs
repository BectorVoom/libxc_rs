//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 797/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk797<F: Float>(t1317: F, t16194: F, t16060: F, t3883: F, t26: F, t1330: F, t16073: F, t16069: F, t11462: F, t16055: F, t16065: F, t4714: F, t16082: F, t1324: F, t494: F, t531: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16195 = t1317 * t16194;
    let t16197 = t3883 * t16060;
    let t16198 = t26 * t16197;
    let t16200 = t1330 * t16073;
    let t16201 = t26 * t16200;
    let t16203 = t3883 * t16069;
    let t16204 = t26 * t16203;
    let t16206 = t11462 * t16055;
    let t16207 = t26 * t16206;
    let t16209 = t3883 * t16065;
    let t16210 = t4714 * t16209;
    let t16212 = t1330 * t16082;
    let t16213 = t26 * t16212;
    let t16215 = t1324 * t16194;
    let t16217 = t494 * t531;
    (t16195, t16198, t16201, t16204, t16207, t16210, t16213, t16215, t16217)
}
