//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2061/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2061(t30: f64, t41154: f64, t1957: f64, t25392: f64, t14495: f64, t689: f64, t25372: f64, t25386: f64, t27357: f64, t14587: f64, t27312: f64, t92838: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t98785 = t41154 * t30;
    let t98799 = t1957 * t25392;
    let t98801 = t14495 * t689;
    let t98803 = 0.14456046980341999104e-1_f64 * t25372 * t98799 * t98801;
    let t98806 = 0.25702851531048074406e-1_f64 * t25386 * t98799 * t98801;
    let t98807 = t1957 * t27357;
    let t98809 = t14587 * t689;
    let t98811 = 0.28912093960683998208e-1_f64 * t25372 * t98807 * t98809;
    let t98814 = 0.51405703062096148812e-1_f64 * t25386 * t98807 * t98809;
    let t98815 = t27312 * t689;
    let t98817 = 0.51405703062096148812e-1_f64 * t92838 * t98815;
    (t98785, t98803, t98806, t98811, t98814, t98815, t98817)
}
