//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1192/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1192(t37848: f64, t37851: f64, t39763: f64, t39771: f64, t39772: f64, t39786: f64, t43230: f64, t43232: f64, t43234: f64, t43238: f64, t43240: f64, t43242: f64) -> f64 {
    let t43244 = -0.42377972951376424087e0_f64 * t37848 - 0.12713391885412927226e1_f64 * t37851 - t39763 + 0.19514881078765566037e-1_f64 * t43230 + 0.43663693315433241792e-2_f64 * t43232 + t39771 + 0.12805040077930161442e0_f64 * t43234 - 0.85366933852867742947e0_f64 * t39772 + 0.11557628986739024751e0_f64 * t43238 + t39786 + 0.21831846657716620896e-2_f64 * t43240 - 0.43663693315433241792e-2_f64 * t43242;
    t43244
}
