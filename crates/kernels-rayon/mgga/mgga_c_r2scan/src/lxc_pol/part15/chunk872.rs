//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 872/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk872(t246: f64, t4873: f64, t5032: f64, t5039: f64, t6036: f64, t6039: f64, t6047: f64, t7028: f64, t7156: f64, t7158: f64, t7160: f64, t7161: f64) -> f64 {
    let t7910 = -t4873 + 0.285764e-1_f64 * t6036 + 0.571528e-1_f64 * t6039 + t6047 - 0.285764e-1_f64 * t246 * t7028 + t7156 + t7158 + t7160 - t5032 - t7161 - t5039;
    t7910
}
