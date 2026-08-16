//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1425/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1425(t33: f64, t3881: f64, t6416: f64, t1113: f64, t1348: f64, t20256: f64, t21956: f64, t2255: f64, t5582: f64, t21955: f64, t1882: f64, t1892: f64, t4003: f64, t5658: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t34 = t33 <= zeta_threshold;
    let t21961 = t3881 * t6416;
    let t21967 = piecewise3(t34, 0.0_f64, 8.0_f64 / 27.0_f64 * t21956 * t1113 + 8.0_f64 / 9.0_f64 * t5582 * t2255 - 2.0_f64 / 9.0_f64 * t21961 * t1113 + 2.0_f64 / 3.0_f64 * t1348 * t20256);
    let t21969 = t21955 / 2.0_f64 + t21967 / 2.0_f64;
    let t21981 = t1892 * t1882;
    let t21990 = t4003 * t5658;
    (t21969, t21981, t21990)
}
