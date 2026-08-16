//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1158/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1158(t121227: f64, t121272: f64, t121275: f64, t121099: f64, t32275: f64, t32707: f64, t121307: f64, t121342: f64, t121346: f64, t121363: f64, t116: f64, t32608: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t122480 = 0.39666484489654411541e-3_f64 * t121227;
    let t122493 = 0.7437465841810202164e-5_f64 * t121272;
    let t122494 = 0.39671442800215618342e-4_f64 * t121275;
    let t122496 = t121099 * t32275 * t32707;
    let t122498 = 0.40155686056505553065e-3_f64 * t121307;
    let t122503 = 0.71396809808466873356e-3_f64 * t121342;
    let t122504 = 0.37645955677973955999e-5_f64 * t121346;
    let t122512 = 0.35702867204846465857e-4_f64 * t121363;
    let t122570 = t32608 * t116;
    (t122480, t122493, t122494, t122496, t122498, t122503, t122504, t122512, t122570)
}
