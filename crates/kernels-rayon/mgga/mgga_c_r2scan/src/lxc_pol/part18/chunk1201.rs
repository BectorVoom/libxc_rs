//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1201/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1201(t10760: f64, t22790: f64, t31064: f64, t37933: f64, t43313: f64, t43316: f64, t43319: f64, t43322: f64, t43324: f64, t43327: f64, t43330: f64, t43332: f64, t43335: f64, t43337: f64) -> f64 {
    let t43340 = t22790 * t10760 * t31064;
    let t43342 = -0.21341733463216935736e0_f64 * t37933 + 0.11557628986739024751e0_f64 * t43313 + 0.13972381860938637374e0_f64 * t43316 + 0.13099107994629972538e-1_f64 * t43319 + 0.46574606203128791245e-1_f64 * t43322 + 0.5200933044032561138e0_f64 * t43324 + 0.43663693315433241792e-2_f64 * t43327 - 0.43341108700271342816e-1_f64 * t43330 - 0.2600466522016280569e0_f64 * t43332 + 0.21831846657716620896e-2_f64 * t43335 + 0.69345773920434148507e0_f64 * t43337 + 0.26198215989259945076e-1_f64 * t43340;
    t43342
}
