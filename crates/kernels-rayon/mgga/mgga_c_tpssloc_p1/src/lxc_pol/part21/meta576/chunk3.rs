//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2298/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2298(t1268: f64, t12725: f64, t1458: f64, t19450: f64, t19451: f64, t19456: f64, t19461: f64, t19534: f64, t2314: f64, t4028: f64, t4072: f64, t5113: f64, t5493: f64, t671: f64, t7676: f64) -> f64 {
    let t19537 = 2.0_f64 * t1268 * t19534 + 4.0_f64 * t12725 * t1458 + 4.0_f64 * t1458 * t19456 + 2.0_f64 * t19451 * t671 + 2.0_f64 * t2314 * t5493 + 4.0_f64 * t4028 * t4072 + 4.0_f64 * t4072 * t7676 + 2.0_f64 * t5113 * t5493 + t19450 + 2.0_f64 * t19461;
    t19537
}
