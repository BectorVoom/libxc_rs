//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1345/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1345(t1165: f64, t13133: f64, t13554: f64, t1799: f64, t19305: f64, t19656: f64, t20319: f64, t2056: f64, t21180: f64, t21227: f64, t21907: f64, t3493: f64, t4347: f64, t5815: f64, t6234: f64, t6323: f64, t69026: f64, t69069: f64, t69072: f64, t71159: f64) -> f64 {
    let t71603 = 2.0_f64 * t1165 * t71159 + 4.0_f64 * t13133 * t6323 + 4.0_f64 * t13554 * t6323 + 4.0_f64 * t1799 * t69026 + 2.0_f64 * t1799 * t69069 + 2.0_f64 * t1799 * t69072 + 4.0_f64 * t19305 * t6323 + 4.0_f64 * t19656 * t6323 + 4.0_f64 * t20319 * t3493 + 4.0_f64 * t20319 * t6234 + 2.0_f64 * t2056 * t21907 + 4.0_f64 * t21180 * t5815 + 2.0_f64 * t21227 * t5815 + 2.0_f64 * t21907 * t4347;
    t71603
}
