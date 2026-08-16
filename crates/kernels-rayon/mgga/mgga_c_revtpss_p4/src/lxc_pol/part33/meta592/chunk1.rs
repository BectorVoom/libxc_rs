//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2008/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2008(t3981: f64, t94443: f64, t25981: f64, t820: f64, t843: f64, t2681: f64, t7262: f64, t1401: f64, t533: f64, t816: f64, t92993: f64, t7259: f64, t9709: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t94444 = t94443 * t3981;
    let t94455 = t820 * t25981 * t843;
    let t94459 = t820 * t7262 * t2681;
    let t94460 = t94459 * t1401;
    let t94471 = t92993 * t533 * t816;
    let t94472 = 455.0_f64 / 1296.0_f64 * t94471;
    let t94473 = t7259 * t9709;
    (t94444, t94455, t94459, t94460, t94472, t94473)
}
