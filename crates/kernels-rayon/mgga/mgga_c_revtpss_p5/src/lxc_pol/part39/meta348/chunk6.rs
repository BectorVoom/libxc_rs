//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1180/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1180(t221: f64, t4019: f64, t5659: f64, t4018: f64, t3989: f64, t5629: f64, t3930: f64, t5661: f64, t5665: f64, t9976: f64, t1412: f64, t1882: f64) -> (f64, f64, f64, f64, f64) {
    let t14036 = t4019 * t221 * t5659;
    let t14038 = 0.25410001404642664112e-4_f64 * t4018 * t14036;
    let t14040 = 0.40015750243531754508e-1_f64 * t3989 * t5629;
    let t14042 = 0.20007875121765877254e-2_f64 * t3930 * t5661;
    let t14043 = t9976 * t5665;
    let t14045 = t1412 * t1882;
    (t14038, t14040, t14042, t14043, t14045)
}
