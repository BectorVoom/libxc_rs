//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1022/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1022(t1419: f64, t3999: f64, t123: f64, t212: f64, t2434: f64, t4089: f64, t138: f64, t2438: f64, t785: f64, t1432: f64, t2470: f64, t4107: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10049 = t3999 * t1419;
    let t10069 = t123 * t2434 * t212;
    let t10070 = t10069 * t4089;
    let t10073 = t138 * t2438 * t785;
    let t10074 = t10073 * t4089;
    let t10098 = t1432 * t4107 * t2470;
    (t10049, t10069, t10070, t10073, t10074, t10098)
}
