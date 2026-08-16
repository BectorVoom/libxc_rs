//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 889/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk889(t6189: f64, t973: f64, t2994: f64, t3001: f64, t4571: f64, t4620: f64, t6094: f64, t6098: f64, t6102: f64, t6114: f64, t6121: f64, t6127: f64, t6129: f64, t6133: f64, t6136: f64, t6139: f64) -> (f64, f64) {
    let t6190 = t6189 * t973;
    let t6205 = -0.1294625e1_f64 * t6114 + 0.258925e1_f64 * t6121 + t2994 + 0.20128333333333333334e0_f64 * t4571 - 0.20128333333333333333e0_f64 * t6094 + 0.60385e0_f64 * t6098 - 0.301925e0_f64 * t6102 + 0.82524375e-1_f64 * t6127 + 0.16504875e0_f64 * t6129 + t3001 + 0.11038e0_f64 * t4620 - 0.27595e-1_f64 * t6133 + 0.16557e0_f64 * t6136 - 0.82785e-1_f64 * t6139;
    (t6190, t6205)
}
