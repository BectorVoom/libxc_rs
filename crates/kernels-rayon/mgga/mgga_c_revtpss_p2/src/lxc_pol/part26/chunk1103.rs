//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1103/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1103(t13846: f64, t1941: f64, t241: f64, t25981: f64, t820: f64, t197: f64, t530: f64, t2013: f64, t8995: f64, t2106: f64, t9593: f64, t198: f64, t205: f64, t2070: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27932 = t1941 * t13846;
    let t27940 = t820 * t25981 * t241;
    let t28166 = t197 * t530;
    let t28167 = t2013 * t28166;
    let t28196 = t2013 * t8995;
    let t28286 = t2106 * t9593;
    let t28291 = t198 * t205 * t2070;
    (t27932, t27940, t28167, t28196, t28286, t28291)
}
