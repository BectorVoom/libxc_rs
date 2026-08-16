//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2739/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2739(t10722: f64, t4345: f64, t40710: f64, t4349: f64, t14834: f64, t9775: f64, t10716: f64, t14857: f64, t2475: f64, t4343: f64, t14832: f64, t2661: f64, t775: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t50383 = t10722 * t4345;
    let t50385 = t40710 * t4349;
    let t50387 = t9775 * t14834;
    let t50389 = t10716 * t14857;
    let t50390 = 0.16262400898971305032e-2_f64 * t50389;
    let t50391 = t2475 * t4343;
    let t50394 = t2661 * t14832 * t50391 * t775;
    (t50383, t50385, t50387, t50390, t50391, t50394)
}
