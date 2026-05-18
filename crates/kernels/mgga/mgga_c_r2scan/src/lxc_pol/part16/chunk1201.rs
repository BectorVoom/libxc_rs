//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1201/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1201<F: Float>(t10760: F, t22790: F, t31064: F, t37933: F, t43313: F, t43316: F, t43319: F, t43322: F, t43324: F, t43327: F, t43330: F, t43332: F, t43335: F, t43337: F) -> F {
    let t43340 = t22790 * t10760 * t31064;
    let t43342 = -F::new(0.21341733463216935736e0) * t37933 + F::new(0.11557628986739024751e0) * t43313 + F::new(0.13972381860938637374e0) * t43316 + F::new(0.13099107994629972538e-1) * t43319 + F::new(0.46574606203128791245e-1) * t43322 + F::new(0.5200933044032561138e0) * t43324 + F::new(0.43663693315433241792e-2) * t43327 - F::new(0.43341108700271342816e-1) * t43330 - F::new(0.2600466522016280569e0) * t43332 + F::new(0.21831846657716620896e-2) * t43335 + F::new(0.69345773920434148507e0) * t43337 + F::new(0.26198215989259945076e-1) * t43340;
    t43342
}
