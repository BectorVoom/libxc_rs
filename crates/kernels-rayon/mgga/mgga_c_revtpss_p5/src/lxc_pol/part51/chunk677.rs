//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 677/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk677(t1936: f64, t2322: f64, t5523: f64, t1312: f64, t7002: f64, t670: f64, t6983: f64, t6985: f64, t1315: f64, t196: f64, t197: f64) -> (f64, f64) {
    let t7226 = 2.0_f64 * t2322 * t1936;
    let t7228 = 2.0_f64 * t5523 * t1936;
    let t7230 = 2.0_f64 * t1312 * t7002;
    let t7231 = 2.0_f64 * t670 * t6985 + t6983 + t7226 + t7228 + t7230;
    let t7234 = t1315 * t196;
    let t7235 = t7234 * t197;
    (t7231, t7235)
}
