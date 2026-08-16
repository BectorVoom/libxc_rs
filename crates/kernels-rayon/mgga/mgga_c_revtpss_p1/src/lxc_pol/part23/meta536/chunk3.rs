//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2074/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2074(t33: f64, t6792: f64, t9617: f64, t3881: f64, t6416: f64, t1113: f64, t1348: f64, t20256: f64, t2255: f64, t5582: f64, t21955: f64, zeta_threshold: f64) -> (f64, f64) {
    let t34 = t33 <= zeta_threshold;
    let t21956 = t9617 * t6792;
    let t21961 = t3881 * t6416;
    let t21967 = piecewise3(t34, 0.0_f64, 8.0_f64 / 27.0_f64 * t21956 * t1113 + 8.0_f64 / 9.0_f64 * t5582 * t2255 - 2.0_f64 / 9.0_f64 * t21961 * t1113 + 2.0_f64 / 3.0_f64 * t1348 * t20256);
    let t21969 = t21955 / 2.0_f64 + t21967 / 2.0_f64;
    (t21956, t21969)
}
