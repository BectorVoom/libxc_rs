//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3826/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3826(t33: f64, t1113: f64, t13701: f64, t14: f64, t20256: f64, t21956: f64, t21961: f64, t27: f64, t3351: f64, t3842: f64, t3881: f64, t46328: f64, t48417: f64, t5582: f64, t580: f64, t6416: f64, t6792: f64, t73449: f64, t9342: f64, t9617: f64, zeta_threshold: f64) -> f64 {
    let t34 = t33 <= zeta_threshold;
    let t73576 = piecewise3(t34, 0.0_f64, -56.0_f64 / 81.0_f64 * t46328 * t6792 * t3842 - 64.0_f64 / 27.0_f64 * t13701 * t73449 + 8.0_f64 / 27.0_f64 * t21956 * t3351 - 16.0_f64 / 9.0_f64 * t3881 * t14 * t27 + 8.0_f64 / 9.0_f64 * t5582 * t580 - 8.0_f64 / 3.0_f64 * t5582 * t9342 + 8.0_f64 / 27.0_f64 * t9617 * t6416 * t3842 - 4.0_f64 / 9.0_f64 * t3881 * t20256 * t1113 - 2.0_f64 / 9.0_f64 * t21961 * t3351 - t48417);
    t73576
}
