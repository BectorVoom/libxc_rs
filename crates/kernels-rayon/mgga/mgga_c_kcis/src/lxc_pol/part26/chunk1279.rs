//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1279/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1279(t28356: f64, t4153: f64, t5663: f64, t1394: f64, t28499: f64, t5649: f64, t101910: f64, t101919: f64, t101922: f64, t101925: f64, t101928: f64, t27636: f64, t28714: f64, t28844: f64, t6176: f64, t77834: f64, t77844: f64, t7968: f64, t7978: f64, t8222: f64, t95009: f64, t99013: f64) -> (f64, f64, f64) {
    let t101931 = t4153 * t28356 * t5663;
    let t101934 = t1394 * t28499 * t5649;
    let t101936 = -0.23168402777777777778e-3_f64 * t99013 * t8222 + 0.46336805555555555556e-3_f64 * t28714 * t28844 + 0.208515625e-2_f64 * t7978 * t6176 * t95009 * t77834 + 0.69505208333333333334e-3_f64 * t7978 * t101910 - 0.13901041666666666667e-2_f64 * t7978 * t6176 * t27636 * t77844 + 0.92754700520833333334e-4_f64 * t7968 * t101910 - 0.7722800925925925926e-4_f64 * t101919 + 0.61905925925925925925e-2_f64 * t101922 - 0.41270617283950617283e-2_f64 * t101925 + 0.12381185185185185185e-1_f64 * t101928 - 0.10317654320987654321e-1_f64 * t101931 + 0.15476481481481481481e-2_f64 * t101934;
    (t101931, t101934, t101936)
}
