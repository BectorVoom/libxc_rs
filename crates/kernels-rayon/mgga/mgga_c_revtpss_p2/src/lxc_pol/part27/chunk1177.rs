//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1177/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1177(t241: f64, t25981: f64, t820: f64, t2022: f64, t3999: f64, t197: f64, t530: f64, t2013: f64, t8995: f64, t2033: f64, t9593: f64, t1936: f64, t670: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27940 = t820 * t25981 * t241;
    let t27980 = t3999 * t2022;
    let t28166 = t197 * t530;
    let t28167 = t2013 * t28166;
    let t28196 = t2013 * t8995;
    let t28197 = t2033 * t9593;
    let t28264 = t670 * t1936;
    (t27940, t27980, t28167, t28196, t28197, t28264)
}
