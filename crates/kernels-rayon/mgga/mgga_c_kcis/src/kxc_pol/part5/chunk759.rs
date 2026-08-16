//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 759/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk759(t1976: f64, t743: f64, t1968: f64, t733: f64, t1971: f64, t738: f64, t167: f64, t4023: f64, t4047: f64, t4050: f64, t4053: f64, t4059: f64, t4081: f64, t4089: f64, t4093: f64, t5654: f64, t5814: f64, t5816: f64, t5820: f64) -> (f64, f64, f64, f64) {
    let t5829 = t743 * t1976;
    let t5831 = t733 * t1968;
    let t5833 = t738 * t1971;
    let t5835 = -0.11955719325063177623e-1_f64 * t5814 + 0.10359077815592613752e-3_f64 * t5816 - 0.23911438650126355246e-1_f64 * t4059 * t167 + 0.10359077815592613752e-3_f64 * t5820 * t167 - 0.23911438650126355246e-1_f64 * t4023 * t5654 + 0.15538616723388920628e-3_f64 * t4093 * t5654 + 0.4684e-2_f64 * t4081 - 0.13208333333333333333e-2_f64 * t4089 + t4047 - t4050 - t4053 - 0.117630625e-4_f64 * t5829 + 0.4684e-2_f64 * t5831 - 0.13208333333333333333e-2_f64 * t5833;
    (t5829, t5831, t5833, t5835)
}
