//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 614/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk614(t1211: f64, t6587: f64, t1477: f64, t476: f64, t52: f64, t475: f64, t467: f64, t1785: f64, t1803: f64, t225: f64, t6564: f64, t480: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6588 = t1211 * t6587;
    let t6593 = 1.0_f64 / t52 / t476 / t1477;
    let t6594 = t475 * t6593;
    let t6595 = t467 * t6594;
    let t6598 = t1785 * t1803;
    let t6601 = t6564 * t225;
    let t6602 = t6601 * t480;
    (t6588, t6593, t6594, t6595, t6598, t6601, t6602)
}
