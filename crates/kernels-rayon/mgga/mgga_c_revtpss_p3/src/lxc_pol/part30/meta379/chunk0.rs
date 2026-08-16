//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1426/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1426(t9404: f64, t2626: f64, t5571: f64, t1856: f64, t2608: f64, t512: f64, t9408: f64, t9411: f64, t9422: f64, t9429: f64, t13612: f64, t13615: f64, t13620: f64, t13622: f64, t13623: f64, t13624: f64, t13625: f64, t4139: f64, t4140: f64, t5536: f64, t5542: f64, t5627: f64, t9394: f64, t9415: f64, t9421: f64, t9427: f64, t9546: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13629 = 4.0_f64 * t9404;
    let t13630 = t5571 * t2626;
    let t13631 = 0.11696447245269292414e1_f64 * t13630;
    let t13632 = t1856 * t2608;
    let t13633 = t512 * t13632;
    let t13634 = 32.0_f64 * t9408;
    let t13635 = 80.0_f64 * t9411;
    let t13636 = 0.23392894490538584828e1_f64 * t9422;
    let t13637 = 2.0_f64 * t9429;
    let t13638 = -6.0_f64 * t13625 * t4139 * t5542 + 12.0_f64 * t4140 * t5536 * t5627 - t13612 - t13615 - t13620 - t13622 + t13623 - t13624 - t13629 + t13631 + t13633 - t13634 + t13635 + t13636 + t13637 + t9394 - t9415 + t9421 - t9427 + t9546;
    (t13629, t13631, t13633, t13634, t13635, t13636, t13637, t13638)
}
