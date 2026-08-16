//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1980/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1980(t7059: f64, t9288: f64, t7064: f64, t25305: f64, t92868: f64, t136: f64, t2457: f64, t7082: f64, t25299: f64, t10073: f64, t1958: f64, t25390: f64, t886: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t92871 = t7059 * t9288;
    let t92873 = 0.39982213492741449076e-1_f64 * t7064 * t92871;
    let t92875 = 0.91399340044406952588e-2_f64 * t25305 * t92868;
    let t92894 = t7082 * t136 * t2457;
    let t92895 = t25299 * t92894;
    let t92905 = t10073 * t25390 * t1958 * t886;
    (t92871, t92873, t92875, t92894, t92895, t92905)
}
