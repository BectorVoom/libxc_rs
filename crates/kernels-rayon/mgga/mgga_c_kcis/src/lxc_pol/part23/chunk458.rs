//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 458/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk458(t3820: f64, t513: f64, t1317: f64, t1416: f64, t3781: f64, t3809: f64, t3793: f64, t3795: f64, t3799: f64, t3803: f64, t3807: f64, t1319: f64, t1410: f64, t456: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3821 = t3820 * t513;
    let t3824 = t1317 * t1416;
    let t3829 = t3820 * t3781;
    let t3831 = t1317 * t3809;
    let t3833 = 0.55033333333333333333e-2_f64 * t3793;
    let t3838 = -0.991e-2_f64 * t3829 + 0.1982e-1_f64 * t3831 + t3833 + 0.27516666666666666666e-2_f64 * t3795 - 0.27516666666666666667e-2_f64 * t3799 + 0.8255e-2_f64 * t3803 - 0.41275e-2_f64 * t3807;
    let t3841 = -t3821 * t3781 / 8.0_f64 + t3824 * t1319 / 2.0_f64 + t1410 * t3809 / 4.0_f64 + t456 * t3838 / 2.0_f64;
    (t3821, t3824, t3829, t3831, t3833, t3838, t3841)
}
