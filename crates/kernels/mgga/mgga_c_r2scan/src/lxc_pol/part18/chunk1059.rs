//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1059/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1059<F: Float>(t261: F, t3299: F, t9366: F, t3594: F, t39745: F, t10760: F, t2147: F, t28005: F, t11727: F, t11748: F, t22790: F, t31064: F, t37933: F, t43313: F, t43316: F, t43319: F, t43322: F, t43324: F, t43327: F) -> (F,) {
    let t43330 = t3299 * t261 * t9366;
    let t43332 = t39745 * t3594;
    let t43335 = t2147 * t10760 * t28005;
    let t43337 = t11748 * t11727;
    let t43340 = t22790 * t10760 * t31064;
    let t43342 = -0.21341733463216935736e0 * t37933 + 0.11557628986739024751e0 * t43313 + 0.13972381860938637374e0 * t43316 + 0.13099107994629972538e-1 * t43319 + 0.46574606203128791245e-1 * t43322 + 0.5200933044032561138e0 * t43324 + 0.43663693315433241792e-2 * t43327 - 0.43341108700271342816e-1 * t43330 - 0.2600466522016280569e0 * t43332 + 0.21831846657716620896e-2 * t43335 + 0.69345773920434148507e0 * t43337 + 0.26198215989259945076e-1 * t43340;
    (t43342,)
}
