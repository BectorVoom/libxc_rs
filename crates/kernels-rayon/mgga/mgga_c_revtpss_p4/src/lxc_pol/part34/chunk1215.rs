//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1215/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1215(t94973: f64, t239: f64, t655: f64, t2339: f64, t624: f64, t10208: f64, t68: f64, t1892: f64, t786: f64, t25877: f64, t1426: f64, t7911: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t94974 = 154.0_f64 / 27.0_f64 * t94973;
    let t94975 = t239 * t655;
    let t94978 = t624 * t2339;
    let t94982 = t68 * t10208;
    let t97699 = t786 * t1892;
    let t97700 = t97699 * t25877;
    let t97783 = t786 * t7911 * t1426;
    (t94974, t94975, t94978, t94982, t97699, t97700, t97783)
}
