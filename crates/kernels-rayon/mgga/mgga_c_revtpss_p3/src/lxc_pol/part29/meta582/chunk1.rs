//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1934/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1934(t1113: f64, t4343: f64, t1583: f64, t3351: f64, t27799: f64, t63164: f64, t4433: f64, t892: f64, t14749: f64, t27763: f64, t14767: f64, t1711: f64, t2408: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t100997 = t1113 * t4343;
    let t101012 = t3351 * t1583;
    let t101016 = t27799 * t63164;
    let t101029 = t892 * t1113 * t4433;
    let t101032 = t27763 * t14749;
    let t101035 = t27763 * t14767;
    let t101040 = t1711 * t2408;
    (t100997, t101012, t101016, t101029, t101032, t101035, t101040)
}
