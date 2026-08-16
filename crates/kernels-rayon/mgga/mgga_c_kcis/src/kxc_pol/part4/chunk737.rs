//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 737/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk737(t1572: f64, t4332: f64, t3793: f64, t3879: f64, t3795: f64, t3799: f64, t3803: f64, t3807: f64, t3829: f64, t3831: f64, t3874: f64, t3876: f64, t3881: f64, t3885: f64, t3888: f64, t3891: f64) -> (f64, f64, f64, f64) {
    let t4333 = t4332 * t1572;
    let t4338 = 0.68863333333333333333e0_f64 * t3793;
    let t4345 = 0.17365833333333333333e0_f64 * t3879;
    let t4350 = -0.17648625e1_f64 * t3829 + 0.3529725e1_f64 * t3831 + t4338 + 0.34431666666666666666e0_f64 * t3795 - 0.34431666666666666667e0_f64 * t3799 + 0.103295e1_f64 * t3803 - 0.516475e0_f64 * t3807 + 0.31558125e0_f64 * t3874 + 0.6311625e0_f64 * t3876 + t4345 + 0.13892666666666666667e0_f64 * t3881 - 0.34731666666666666667e-1_f64 * t3885 + 0.20839e0_f64 * t3888 - 0.104195e0_f64 * t3891;
    (t4333, t4338, t4345, t4350)
}
