//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3246/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3246(t33: f64, t1113: f64, t6416: f64, t580: f64, t1348: f64, t13701: f64, t13704: f64, t20256: f64, t21956: f64, t2255: f64, t22778: f64, t22783: f64, t3881: f64, t46328: f64, t5582: f64, t81123: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t34 = t33 <= zeta_threshold;
    let t85426 = t6416 * t1113;
    let t85429 = t580 * t6416;
    let t85440 = piecewise3(t34, 0.0_f64, -56.0_f64 / 81.0_f64 * t46328 * t22778 * t1113 - 16.0_f64 / 9.0_f64 * t21956 * t2255 + 8.0_f64 / 9.0_f64 * t13701 * t85426 + 4.0_f64 / 3.0_f64 * t13704 * t85429 - 2.0_f64 / 3.0_f64 * t5582 * t20256 - 2.0_f64 / 9.0_f64 * t3881 * t22783 * t1113 + 2.0_f64 / 3.0_f64 * t1348 * t81123);
    (t85426, t85429, t85440)
}
