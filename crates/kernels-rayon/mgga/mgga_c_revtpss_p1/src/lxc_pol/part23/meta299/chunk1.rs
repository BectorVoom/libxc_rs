//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1550/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1550(t11940: f64, t366: f64, t1053: f64, t3204: f64, t1021: f64, t3201: f64, t1054: f64, t2434: f64, t371: f64, t373: f64, t367: f64, t1065: f64, t675: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11941 = t11940 * t366;
    let t11947 = t3204 * t1053;
    let t11956 = t1021 * t3201;
    let t11967 = t1054 * t3201;
    let t11970 = t371 * t2434 * t373;
    let t11972 = 0.63517063878621832551e-4_f64 * t367 * t11970;
    let t11986 = t675 * t1065;
    (t11941, t11947, t11956, t11967, t11970, t11972, t11986)
}
