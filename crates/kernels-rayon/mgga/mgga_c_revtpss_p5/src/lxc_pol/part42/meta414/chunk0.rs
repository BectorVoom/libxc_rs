//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1468/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1468(t4292: f64, t94: f64, t1513: f64, t665: f64, t93: f64, t5920: f64, t1501: f64, t1518: f64, t2339: f64, t625: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27126 = t94 * t4292;
    let t28036 = t1513 * t665;
    let t28219 = t93 * t4292;
    let t29508 = t94 * t5920;
    let t30138 = t1501 * t1518;
    let t30143 = t93 * t5920;
    let t31027 = t625 * t2339;
    (t27126, t28036, t28219, t29508, t30138, t30143, t31027)
}
