//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2630/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2630(t30: f64, t1868: f64, t9940: f64, t5577: f64, t588: f64, t1344: f64, t13687: f64, t13690: f64, t1468: f64, t2: f64, t22: f64, t3874: f64, t46310: f64, t48165: f64, t48168: f64, t48174: f64, t48177: f64, t5574: f64, t580: f64, t605: f64, t9336: f64, t9344: f64, t9605: f64, zeta_threshold: f64) -> (f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t48347 = t9940 * t1868;
    let t48394 = 16.0_f64 * t5577 * t588;
    let t48396 = piecewise3(t31, 0.0_f64, -56.0_f64 / 81.0_f64 * t46310 * t1468 * t9336 + 16.0_f64 / 9.0_f64 * t9605 * t2 * t48165 + 8.0_f64 / 9.0_f64 * t13687 * t48168 - 4.0_f64 / 3.0_f64 * t3874 * t580 * t605 + 4.0_f64 * t13690 * t48174 - 4.0_f64 / 3.0_f64 * t13690 * t48177 - 2.0_f64 / 9.0_f64 * t5574 * t9344 - 8.0_f64 * t1344 * t22 + t48394);
    (t48347, t48396)
}
