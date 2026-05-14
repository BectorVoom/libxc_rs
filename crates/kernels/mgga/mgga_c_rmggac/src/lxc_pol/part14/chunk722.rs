//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 722/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk722<F: Float>(t36942: F, t1323: F, t1326: F, t14267: F, t35253: F, t36940: F, t68: F, t36268: F, t7198: F, t7197: F, t899: F, t271: F, t3899: F, t638: F, t641: F, t1347: F, t2128: F) -> (F, F, F, F, F, F, F) {
    let t36943 = 0.13010691197123848594e-3 * t36942;
    let t36945 = t1323 * t1326 * t14267;
    let t36948 = t36945 * t35253 * t68 * t36940;
    let t36976 = t7198 * t36268;
    let t36978 = t899 * t7197;
    let t36983 = t638 * t3899 * t271 * t641;
    let t36984 = 0.69557008413371175709e-2 * t36983;
    let t36992 = t1347 * t2128;
    (t36943, t36945, t36948, t36976, t36978, t36984, t36992)
}
