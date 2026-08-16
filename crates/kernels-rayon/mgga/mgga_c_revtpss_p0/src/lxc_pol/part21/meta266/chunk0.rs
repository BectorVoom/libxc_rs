//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1474/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1474(t521: f64, t9342: f64, t14: f64, t588: f64, t2496: f64, t4038: f64, t123: f64, t1330: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9854 = 24.0_f64 * t9342 * t521;
    let t9855 = t14 * t588;
    let t9856 = t9855 * t521;
    let t9857 = 144.0_f64 * t9856;
    let t9858 = t4038 * t2496;
    let t9859 = 0.51947577317044391276e2_f64 * t9858;
    let t9860 = t1330 * t123;
    (t9854, t9855, t9856, t9857, t9858, t9859, t9860)
}
