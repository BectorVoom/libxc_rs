//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1186/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1186(t47672: f64, t8598: f64, t28196: f64, t28198: f64, t28056: f64, t8634: f64, t32129: f64, t7898: f64, t2007: f64, t28042: f64, t651: f64, t13426: f64, t8461: f64) -> (f64, f64, f64, f64, f64) {
    let t127354 = t8598 * t47672;
    let t127357 = 6.0_f64 * t28196 * t127354 * t28198;
    let t127359 = 4.0_f64 * t8634 * t28056;
    let t127361 = 2.0_f64 * t7898 * t32129;
    let t127363 = t651 * t2007 * t28042;
    let t127365 = t13426 * t8461;
    (t127357, t127359, t127361, t127363, t127365)
}
