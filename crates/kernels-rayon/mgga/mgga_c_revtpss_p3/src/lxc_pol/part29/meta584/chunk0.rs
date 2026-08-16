//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1936/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1936(t27799: f64, t61155: f64, t1711: f64, t2832: f64, t1113: f64, t4537: f64, t13392: f64, t603: f64, t13396: f64, t13405: f64, t4237: f64, t644: f64, t77: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t101086 = t27799 * t61155;
    let t101093 = t1711 * t2832;
    let t101099 = t1113 * t4537;
    let t101129 = t603 * t13392;
    let t101132 = t603 * t13396;
    let t101139 = t603 * t13405;
    let t101156 = t77 * t4237 * t644;
    (t101086, t101093, t101099, t101129, t101132, t101139, t101156)
}
