//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1183/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1183(t1577: f64, t3308: f64, t9547: f64, t39549: f64, t41439: f64, t43115: f64, t43117: f64, t43120: f64, t43123: f64, t43126: f64, t43130: f64, t43133: f64, t43135: f64, t43138: f64) -> f64 {
    let t43141 = t1577 * t3308 * t9547;
    let t43143 = 0.54878743191129263322e-1_f64 * t43115 - t39549 - 0.54878743191129263322e-2_f64 * t43117 - t41439 - 0.13099107994629972538e-1_f64 * t43120 + 0.43663693315433241792e-2_f64 * t43123 + 0.21831846657716620896e-2_f64 * t43126 + 0.21831846657716620896e-2_f64 * t43130 - 0.13972381860938637374e0_f64 * t43133 + 0.2600466522016280569e0_f64 * t43135 + 0.13002332610081402845e0_f64 * t43138 + 0.26004665220162805689e0_f64 * t43141;
    t43143
}
