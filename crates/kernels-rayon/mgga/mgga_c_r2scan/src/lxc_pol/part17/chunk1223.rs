//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1223/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1223(t39558: f64, t41435: f64, t43115: f64, t43117: f64, t43120: f64, t43123: f64, t43126: f64, t43130: f64, t43133: f64, t43135: f64, t43138: f64, t43141: f64) -> f64 {
    let t44242 = 0.10975748638225852664e0_f64 * t43115 - t41435 - 0.10975748638225852664e-1_f64 * t43117 - 0.90044238659382329742e0_f64 * t39558 - 0.26198215989259945077e-1_f64 * t43120 + 0.87327386630866483588e-2_f64 * t43123 + 0.43663693315433241794e-2_f64 * t43126 + 0.43663693315433241794e-2_f64 * t43130 - 0.27944763721877274748e0_f64 * t43133 + 0.5200933044032561138e0_f64 * t43135 + 0.2600466522016280569e0_f64 * t43138 + 0.52009330440325611378e0_f64 * t43141;
    t44242
}
