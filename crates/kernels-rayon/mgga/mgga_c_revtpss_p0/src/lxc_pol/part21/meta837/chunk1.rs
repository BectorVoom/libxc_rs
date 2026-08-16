//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3138/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3138(t16658: f64, t44101: f64, t12243: f64, t16665: f64, t16669: f64, t44012: f64, t3384: f64, t3427: f64, t5105: f64, t12571: f64, t5198: f64, t1196: f64, t12485: f64, t3524: f64, t5180: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t57833 = 0.28947563097646563121e3_f64 * t44101 * t16658;
    let t57835 = 0.48245938496077605201e2_f64 * t12243 * t16665;
    let t57837 = 0.1551780387578202009e4_f64 * t44012 * t16669;
    let t57840 = 6.0_f64 * t3384 * t5105 * t3427;
    let t57842 = 0.35089341735807877242e1_f64 * t12571 * t5198;
    let t57846 = 0.31168546390226634765e3_f64 * t1196 * t12485 * t5180 * t3524;
    (t57833, t57835, t57837, t57840, t57842, t57846)
}
