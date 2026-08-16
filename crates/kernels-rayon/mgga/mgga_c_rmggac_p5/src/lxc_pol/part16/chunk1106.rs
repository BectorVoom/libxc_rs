//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1106/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1106(t10459: f64, t333: f64, t1356: f64, t1668: f64, t43722: f64, t43745: f64, t43746: f64, t43974: f64, t44405: f64, t47306: f64, t47310: f64, t47316: f64, t47321: f64, t47325: f64, t47327: f64, t47331: f64, t47333: f64, t47335: f64, t47340: f64, t530: f64, t5888: f64, t884: f64, t9639: f64) -> (f64, f64) {
    let t48976 = t10459 * t333;
    let t48990 = 0.85129199786595678799e-5_f64 * t47306 - 0.77813409179935112652e-4_f64 * t47310 - t43722 + 0.2993560425465952141e-1_f64 * t47316 - 0.23948483403727617128e0_f64 * t1356 * t43974 * t5888 + 0.59871208509319042821e-1_f64 * t884 * t48976 - 0.5107751987195740728e-4_f64 * t47321 - 0.3405167991463827152e-4_f64 * t47325 + 0.1702583995731913576e-4_f64 * t47327 + 0.1702583995731913576e-4_f64 * t47331 + 0.212822999466489197e-4_f64 * t47333 + 0.212822999466489197e-4_f64 * t47335 - 0.4726e1_f64 * t530 * t44405 - 0.4726e1_f64 * t1668 * t9639 + t43745 + t43746 + 0.17961362552795712846e0_f64 * t47340;
    (t48976, t48990)
}
