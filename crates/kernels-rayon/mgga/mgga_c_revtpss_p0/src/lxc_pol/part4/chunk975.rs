//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 975/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk975(t2735: f64, t4086: f64, t3994: f64, t808: f64, t521: f64, t9342: f64, t14: f64, t588: f64, t2496: f64, t4038: f64, t123: f64, t1330: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9845 = t2735 * t4086;
    let t9846 = t808 * t3994;
    let t9847 = t9845 * t9846;
    let t9854 = 24.0_f64 * t9342 * t521;
    let t9855 = t14 * t588;
    let t9856 = t9855 * t521;
    let t9858 = t4038 * t2496;
    let t9860 = t1330 * t123;
    (t9845, t9847, t9854, t9856, t9858, t9860)
}
