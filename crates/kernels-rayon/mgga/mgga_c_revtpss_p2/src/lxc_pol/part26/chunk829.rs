//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 829/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk829(t10626: f64, t10627: f64, t775: f64, t853: f64, t2430: f64, t10489: f64, t832: f64, t10618: f64, t227: f64, t229: f64, t2634: f64, t2639: f64, t2642: f64, t4415: f64, t830: f64, t833: f64) -> (f64, f64) {
    let t10628 = t10626 * t10627;
    let t10631 = t853 * t775;
    let t10632 = t10631 * t2430;
    let t10635 = t832 * t10489;
    let t10638 = -t10618 * t229 + 60.0_f64 * t10628 * t227 - 36.0_f64 * t10632 * t4415 + 3.0_f64 * t10635 * t227 + 9.0_f64 * t2634 * t833 - 36.0_f64 * t2639 * t830 + 9.0_f64 * t2642 * t830;
    (t10631, t10638)
}
