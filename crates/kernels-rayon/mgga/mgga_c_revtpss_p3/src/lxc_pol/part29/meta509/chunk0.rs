//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1827/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1827(t1927: f64, t2315: f64, t2247: f64, t2259: f64, t2411: f64, t605: f64, t268: f64, t41040: f64, t837: f64, t1032: f64, t2760: f64, t867: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t92584 = t1927 * t2315;
    let t92588 = t2247 * t2259;
    let t92790 = t2411 * t605;
    let t92840 = t268 * t41040 * t837;
    let t92888 = t2760 * t1032;
    let t92889 = t92888 * t867;
    (t92584, t92588, t92790, t92840, t92888, t92889)
}
