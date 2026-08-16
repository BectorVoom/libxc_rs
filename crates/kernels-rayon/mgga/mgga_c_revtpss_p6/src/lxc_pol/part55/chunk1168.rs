//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1168/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1168(t18227: f64, t8460: f64, t27123: f64, t28219: f64, t28019: f64, t4147: f64, t32110: f64, t7732: f64, t1353: f64, t7933: f64, t1907: f64, t7311: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t125386 = t18227 * t8460;
    let t125388 = t27123 * t8460;
    let t125390 = t28219 * t8460;
    let t125428 = t4147 * t28019;
    let t125507 = 2.0_f64 * t7732 * t32110;
    let t125559 = t7933 * t1353;
    let t125563 = t1907 * t7311;
    (t125386, t125388, t125390, t125428, t125507, t125559, t125563)
}
