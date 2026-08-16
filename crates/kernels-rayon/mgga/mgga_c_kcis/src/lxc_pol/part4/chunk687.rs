//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 687/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk687(t1356: f64, t3918: f64, t3919: f64, t3793: f64, t3879: f64, t3795: f64, t3799: f64, t3803: f64, t3807: f64, t3829: f64, t3831: f64, t3874: f64, t3876: f64, t3881: f64, t3885: f64, t3888: f64, t3891: f64) -> (f64, f64, f64, f64) {
    let t3921 = t3918 * t3919 * t1356;
    let t3926 = 0.40256666666666666667e0_f64 * t3793;
    let t3933 = 0.137975e0_f64 * t3879;
    let t3938 = -0.1294625e1_f64 * t3829 + 0.258925e1_f64 * t3831 + t3926 + 0.20128333333333333334e0_f64 * t3795 - 0.20128333333333333333e0_f64 * t3799 + 0.60385e0_f64 * t3803 - 0.301925e0_f64 * t3807 + 0.82524375e-1_f64 * t3874 + 0.16504875e0_f64 * t3876 + t3933 + 0.11038e0_f64 * t3881 - 0.27595e-1_f64 * t3885 + 0.16557e0_f64 * t3888 - 0.82785e-1_f64 * t3891;
    (t3921, t3926, t3933, t3938)
}
