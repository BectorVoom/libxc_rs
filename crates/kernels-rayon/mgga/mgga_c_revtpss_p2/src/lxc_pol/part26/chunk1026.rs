//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1026/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1026(t13225: f64, t3: f64, t2327: f64, t670: f64, t116: f64, t2371: f64, t10259: f64, t117: f64, t1459: f64, t1461: f64, t4158: f64, t4162: f64, t4165: f64, t572: f64, t573: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13226 = t3 * t13225;
    let t13232 = param_d * t13225;
    let t13240 = t2327 * t670;
    let t13243 = t116 * t670;
    let t13244 = t13243 * t2371;
    let t13247 = t117 * t10259;
    let t13250 = t13232 * t573 + 6.0_f64 * t13240 * t572 + 18.0_f64 * t13244 * t572 + 3.0_f64 * t13247 * t572 + 18.0_f64 * t1459 * t4162 + 9.0_f64 * t1459 * t4165 + 9.0_f64 * t1461 * t4158;
    (t13226, t13232, t13240, t13244, t13247, t13250)
}
