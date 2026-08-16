//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1189/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1189(t37393: f64, t37398: f64, t37401: f64, t37407: f64, t37413: f64, t37415: f64, t39326: f64, t39330: f64, t39334: f64, t39338: f64, t39342: f64, t39344: f64, t39347: f64, t39351: f64, t40271: f64) -> f64 {
    let t40272 = -t39326 + t39330 + t39334 - t39338 - t39342 + t39344 + t39347 - 0.86737941314158990624e-4_f64 * t37393 - t37398 + 0.92232789896410962678e-3_f64 * t37401 - t39351 + t37407 + t37413 - t37415 - t40271;
    t40272
}
