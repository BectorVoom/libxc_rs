//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1352/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1352(t1385: f64, t6281: f64, t5709: f64, t94274: f64, t1650: f64, t27356: f64, t5737: f64, t167: f64, t16892: f64, t1943: f64, t101871: f64, t101922: f64, t101925: f64, t103073: f64, t20984: f64, t21655: f64, t27369: f64, t27438: f64, t27453: f64, t27459: f64, t29284: f64, t3984: f64, t59414: f64, t7908: f64) -> (f64, f64, f64, f64) {
    let t103101 = t6281 * t1385;
    let t103103 = t5709 * t94274 * t103101;
    let t103114 = t5709 * t27356 * t1650 * t5737;
    let t103119 = t16892 * t27356 * t167 * t1943;
    let t103132 = 0.27636574074074074073e-2_f64 * t101871 - 0.46336805555555555556e-3_f64 * t7908 * t3984 * t27453 * t59414 - 0.46336805555555555556e-3_f64 * t7908 * t103103 - 0.13901041666666666667e-2_f64 * t7908 * t5709 * t27438 * t20984 + 0.46336805555555555556e-3_f64 * t27459 * t29284 + 0.46336805555555555556e-3_f64 * t7908 * t103114 - 0.92673611111111111112e-3_f64 * t7908 * t103119 - 0.61836467013888888889e-4_f64 * t27369 * t103103 - 0.18534722222222222222e-2_f64 * t7908 * t16892 * t27453 * t21655 - 0.92754700520833333333e-4_f64 * t27369 * t103073 + 0.88437037037037037034e-2_f64 * t101922 - 0.58958024691358024689e-2_f64 * t101925;
    (t103101, t103114, t103119, t103132)
}
