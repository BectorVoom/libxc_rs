//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1989/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1989(t25299: f64, t92894: f64, t10073: f64, t1958: f64, t25390: f64, t886: f64, t1955: f64, t25308: f64, t2769: f64, t7049: f64, t786: f64, t867: f64) -> (f64, f64, f64, f64) {
    let t92895 = t25299 * t92894;
    let t92905 = t10073 * t25390 * t1958 * t886;
    let t92917 = t1955 * t25308 * t2769;
    let t92921 = t786 * t7049 * t867;
    (t92895, t92905, t92917, t92921)
}
