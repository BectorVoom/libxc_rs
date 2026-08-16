//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 851/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk851(t10259: f64, t508: f64, t3813: f64, t670: f64, t10: f64, t580: f64, t22: f64, t576: f64, t15: f64, t588: f64, t11: f64, t2: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10260 = t508 * t10259;
    let t10263 = t3813 * t670;
    let t10270 = t10 * t580;
    let t10271 = 12.0_f64 * t10270;
    let t10272 = t576 * t22;
    let t10273 = 36.0_f64 * t10272;
    let t10275 = 24.0_f64 * t15 * t588;
    let t10276 = t11 * t2;
    (t10260, t10263, t10271, t10273, t10275, t10276)
}
