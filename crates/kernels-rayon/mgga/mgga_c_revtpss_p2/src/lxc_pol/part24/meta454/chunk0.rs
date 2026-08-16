//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1421/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1421(t15014: f64, t9303: f64, t10982: f64, t1568: f64, t9646: f64, t14986: f64, t2453: f64, t14567: f64, t14557: f64, t4519: f64, t9292: f64, t2798: f64, t4499: f64, t9288: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t51237 = t9303 * t15014;
    let t51246 = t9646 * t1568 * t10982;
    let t51258 = t2453 * t14986;
    let t51297 = t2453 * t14567;
    let t51390 = t9303 * t14557;
    let t51403 = t9292 * t4519;
    let t51408 = t2798 * t4499 * t9288;
    (t51237, t51246, t51258, t51297, t51390, t51403, t51408)
}
