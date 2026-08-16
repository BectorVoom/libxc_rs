//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 901/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk901(t106: f64, t797: f64, t8299: f64, t97: f64, t4873: f64, t5032: f64, t5039: f64, t7141: f64, t7144: f64, t7148: f64, t7149: f64, t7150: f64, t7156: f64, t7158: f64, t7160: f64, t7161: f64) -> f64 {
    let t8302 = t97 * t106 * t8299 * t797;
    let t8303 = t7141 - t7144 + t7148 + t7149 + t7150 - t4873 - t8302 + t7156 + t7158 + t7160 - t5032 - t7161 - t5039;
    t8303
}
