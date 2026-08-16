//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2009/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2009(t10073: f64, t25308: f64, t25403: f64, t25402: f64, t7048: f64, t7056: f64, t233: f64, t41077: f64, t25348: f64, t689: f64, t25411: f64, t1955: f64, t92888: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t93112 = t10073 * t25308 * t25403;
    let t93116 = t10073 * t7056 * t25402 * t7048;
    let t93118 = t41077 * t233;
    let t93123 = t25348 * t689;
    let t93124 = t25411 * t93123;
    let t93126 = t1955 * t92888;
    (t93112, t93116, t93118, t93123, t93124, t93126)
}
