//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1215/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1215(t92977: f64, t93018: f64, t93057: f64, t93097: f64, t231: f64, t92883: f64, t10073: f64, t25308: f64, t25403: f64, t25402: f64, t7048: f64, t7056: f64) -> (f64, f64, f64, f64) {
    let t93099 = t92977 + t93018 + t93057 + t93097;
    let t93104 = t92883 * t231;
    let t93112 = t10073 * t25308 * t25403;
    let t93116 = t10073 * t7056 * t25402 * t7048;
    (t93099, t93104, t93112, t93116)
}
