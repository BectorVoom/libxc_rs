//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3678/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3678(t12361: f64, t20577: f64, t20580: f64, t44101: f64, t20641: f64, t12243: f64, t20645: f64, t1149: f64, t20448: f64, t3384: f64, t20447: f64, t3435: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t69581 = 8.0_f64 * t12361 * t20577;
    let t69583 = 0.19298375398431042081e3_f64 * t44101 * t20580;
    let t69585 = 4.0_f64 * t12361 * t20641;
    let t69587 = 0.32163958997385070134e2_f64 * t12243 * t20645;
    let t69590 = 4.0_f64 * t3384 * t20448 * t1149;
    let t69591 = t20447 * t3435;
    (t69581, t69583, t69585, t69587, t69590, t69591)
}
