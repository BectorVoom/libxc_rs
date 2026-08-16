//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 684/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk684(t468: f64, t3862: f64, t3899: f64, t3793: f64, t3795: f64, t3799: f64, t3803: f64, t3807: f64, t482: f64, t1341: f64, t45: f64, t1346: f64, t478: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3900 = t468 * t468;
    let t3901 = 1.0_f64 / t3900;
    let t3902 = t3862 * t3901;
    let t3904 = 0.16081824322151104822e2_f64 * t3899 * t3902;
    let t3905 = 0.12361111111111111111e-1_f64 * t3793;
    let t3910 = t3905 + 0.61805555555555555556e-2_f64 * t3795 - 0.61805555555555555555e-2_f64 * t3799 + 0.18541666666666666667e-1_f64 * t3803 - 0.92708333333333333333e-2_f64 * t3807;
    let t3911 = t3910 * t482;
    let t3914 = t45 * t1341;
    let t3917 = t1346 * t478;
    (t3900, t3901, t3902, t3904, t3905, t3910, t3911, t3914, t3917)
}
