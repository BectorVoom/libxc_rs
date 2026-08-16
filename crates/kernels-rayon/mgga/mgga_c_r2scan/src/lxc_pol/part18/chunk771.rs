//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 771/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk771(t546: f64, t7244: f64, t565: f64, t6212: f64, t938: f64, t6211: f64, t6475: f64, t910: f64, t6480: f64, t2604: f64, t5148: f64, t5147: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7245 = t546 * t7244;
    let t7250 = t565 * t7244;
    let t7257 = t6212 * t938;
    let t7258 = t6211 * t7257;
    let t7259 = t6475 * t7258;
    let t7261 = t6212 * t910;
    let t7262 = t6211 * t7261;
    let t7263 = t6480 * t7262;
    let t7297 = t5148 * t2604;
    let t7298 = t5147 * t7297;
    (t7245, t7250, t7257, t7258, t7259, t7261, t7262, t7263, t7298)
}
