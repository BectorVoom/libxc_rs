//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 928/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk928(t1312: f64, t1518: f64, t2322: f64, t4246: f64, t4248: f64, t4292: f64, t5523: f64, t670: f64, t1450: f64, t1907: f64, t198: f64, t530: f64) -> (f64, f64, f64) {
    let t5528 = 2.0_f64 * t1312 * t4292 + 2.0_f64 * t1518 * t2322 + 2.0_f64 * t1518 * t5523 + 2.0_f64 * t4248 * t670 + t4246;
    let t5532 = t1907 * t1450;
    let t5536 = t198 * t530;
    (t5528, t5532, t5536)
}
