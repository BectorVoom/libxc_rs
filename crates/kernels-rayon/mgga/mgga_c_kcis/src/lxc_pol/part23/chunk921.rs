//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 921/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk921(t522: f64, t531: f64, t1445: f64, t5654: f64, t12065: f64, t3754: f64, t822: f64, t5851: f64, t733: f64, t5854: f64, t10138: f64, t11920: f64, t11951: f64, t12011: f64, t12013: f64, t12015: f64, t12026: f64, t12049: f64, t12070: f64, t16685: f64, t16694: f64, t16782: f64, t317: f64, t323: f64, t333: f64, t3750: f64, t4023: f64, t4051: f64, t4093: f64, t8291: f64) -> f64 {
    let t17126 = t522 * t531;
    let t17137 = 0.47822877300252710492e-1_f64 * t1445 * t5654;
    let t17143 = 0.62154466893555682512e-3_f64 * t12065 * t5654;
    let t17146 = t822 * t3754;
    let t17150 = 0.18736e-1_f64 * t733 * t5851;
    let t17151 = t733 * t5854;
    let t17153 = -0.13208333333333333333e-2_f64 * t12011 + 0.88055555555555555553e-3_f64 * t12013 + 0.26416666666666666666e-2_f64 * t12015 + 0.23526125e-4_f64 * t12026 - 0.1585e-2_f64 * t323 * t8291 * t3750 - 0.10082625e-4_f64 * t333 * t10138 * t17126 + 0.7026e-2_f64 * t317 * t4051 + 0.71734315950379065738e-1_f64 * t11920 * t16694 - 0.62154466893555682512e-3_f64 * t12070 * t16694 + t17137 - 0.23911438650126355246e-1_f64 * t4023 * t16685 + 0.95645754600505420984e-1_f64 * t11951 * t16782 - t17143 + 0.15538616723388920628e-3_f64 * t4093 * t16685 - 0.62154466893555682512e-3_f64 * t17146 * t16782 - t17150 - 0.31226666666666666666e-2_f64 * t17151 - t12049;
    t17153
}
