//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 141/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk141(t498: f64, t499: f64, t493: f64, t489: f64, t475: f64, t303: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t500 = t498 * t499;
    let t501 = t493 * t500;
    let t503 = 1.0_f64 + t489 / 16.0_f64 - t501 / 256.0_f64;
    let t504 = 1.0_f64 / t503;
    let t505 = t475 * t504;
    let t507 = 1.0_f64 + 0.5137e-1_f64 * t303;
    (t500, t501, t503, t504, t505, t507)
}
