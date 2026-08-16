//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 668/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk668(t3316: f64, t811: f64, t1347: f64, t2183: f64, t809: f64, t2188: f64, t1336: f64, t2194: f64, t791: f64, t2167: f64, t2198: f64, t3300: f64, t3311: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3318 = 1.0_f64 * t3316 * t811;
    let t3320 = 1.0_f64 * t2183 * t1347;
    let t3321 = t1347 * t809;
    let t3323 = 2.0_f64 * t2188 * t3321;
    let t3324 = t2194 * t1336;
    let t3325 = t3324 * t791;
    let t3329 = t2198 - t2167 / 3.0_f64 - t3300 / 3.0_f64 + t3311;
    (t3318, t3320, t3321, t3323, t3324, t3325, t3329)
}
