//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 710/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk710(t9836: f64, t2466: f64, t4985: f64, t1923: f64, t2265: f64, t9846: f64, t9848: f64, t9850: f64, t9861: f64, t9865: f64, t9870: f64, t9933: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10314 = 0.13637330827122670865e-1_f64 * t9836;
    let t10315 = t4985 * t2466;
    let t10316 = 0.11974241701863808564e0_f64 * t10315;
    let t10317 = t1923 * t2265;
    let t10318 = 0.2363e1_f64 * t10317;
    let t10319 = 0.212822999466489197e-4_f64 * t9846;
    let t10320 = 0.1702583995731913576e-4_f64 * t9848;
    let t10321 = 0.212822999466489197e-4_f64 * t9850;
    let t10322 = 0.11974241701863808564e0_f64 * t9861;
    let t10323 = 0.40911992481368012596e-1_f64 * t9865;
    let t10324 = 0.5987120850931904282e-1_f64 * t9870;
    let t10325 = 0.1702583995731913576e-4_f64 * t9933;
    (t10314, t10316, t10318, t10319, t10320, t10321, t10322, t10323, t10324, t10325)
}
