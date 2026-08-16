//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1141/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1141(t27254: f64, t27256: f64, t28034: f64, t27924: f64, t27926: f64, t27929: f64, t27937: f64, t27955: f64, t1450: f64, t6816: f64, t7237: f64, t2014: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28336 = 0.28582678745379824648e-4_f64 * t27254;
    let t28337 = 0.16006300097412701803e-1_f64 * t27256;
    let t28679 = 2.0_f64 / 3.0_f64 * t28034;
    let t28872 = 0.2032800112371413129e-3_f64 * t27924;
    let t28873 = 0.16006300097412701803e-1_f64 * t27926;
    let t28874 = 0.28582678745379824648e-4_f64 * t27929;
    let t28877 = 0.11433071498151929859e-3_f64 * t27937;
    let t28885 = 7.0_f64 / 72.0_f64 * t27955;
    let t29494 = t1450 * t6816;
    let t29495 = t7237 * t29494;
    let t29497 = 3.0_f64 * t2014 * t29495;
    (t28336, t28337, t28679, t28872, t28873, t28874, t28877, t28885, t29494, t29495, t29497)
}
