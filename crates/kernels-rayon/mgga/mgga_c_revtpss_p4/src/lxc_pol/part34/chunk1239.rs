//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1239/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1239(t30160: f64, t575: f64, t116: f64, t30004: f64, t1518: f64, t1936: f64, t29568: f64, t5891: f64, t94978: f64, t25823: f64, t5915: f64, t29694: f64, t689: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t105814 = t30160 * t575;
    let t105819 = t116 * t30004;
    let t105823 = t1518 * t1936;
    let t105866 = t29568 * t116;
    let t105870 = t94978 * t5891;
    let t105878 = t25823 * t5915;
    let t105933 = t29694 * t689;
    (t105814, t105819, t105823, t105866, t105870, t105878, t105933)
}
