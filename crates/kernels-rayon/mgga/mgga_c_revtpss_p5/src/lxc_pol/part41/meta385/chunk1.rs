//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1277/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1277(t19649: f64, t906: f64, t1042: f64, t3172: f64, t6301: f64, t1041: f64, t5819: f64, t606: f64) -> (f64, f64, f64) {
    let t19650 = t19649 * t906;
    let t19651 = t1042 * t19650;
    let t19658 = t3172 * t6301;
    let t19659 = t1041 * t19658;
    let t19661 = t5819 * t606;
    (t19651, t19659, t19661)
}
