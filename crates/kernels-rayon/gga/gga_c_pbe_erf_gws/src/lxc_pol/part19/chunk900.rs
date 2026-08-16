//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 900/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk900(t2858: f64, t2873: f64, t3649: f64, t485: f64, t395: f64, t3652: f64, t9779: f64, t9781: f64, t9784: f64, t9789: f64, t9794: f64, t9796: f64, t9799: f64, t9802: f64) -> (f64, f64, f64, f64) {
    let t10046 = t2858 * t2873;
    let t10049 = t485 * t3649;
    let t10050 = t10049 * t395;
    let t10051 = 0.97434166666666666667e0_f64 * t10050;
    let t10052 = t485 * t3652;
    let t10053 = t10052 * t395;
    let t10054 = 0.48717083333333333333e0_f64 * t10053;
    let t10063 = 4.0_f64 / 27.0_f64 * t9779 - 4.0_f64 / 9.0_f64 * t9781 - t9784 / 9.0_f64 + t9789 / 3.0_f64 + 4.0_f64 / 27.0_f64 * t9794 + 4.0_f64 / 9.0_f64 * t9796 - t9799 / 9.0_f64 + t9802 / 3.0_f64;
    (t10046, t10051, t10054, t10063)
}
