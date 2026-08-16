//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1254/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1254(t30: f64, t525: f64, t2: f64, t22: f64, t33: f64, t527: f64, t2490: f64, t737: f64, t2492: f64, t744: f64) -> (f64, f64, f64, f64, f64) {
    let t9335 = 1.0_f64 / t525 / t30;
    let t9342 = t2 * t22;
    let t9350 = 1.0_f64 / t527 / t33;
    let t9367 = 1.0_f64 / t2490 / t737;
    let t9368 = t2492 * t744;
    (t9335, t9342, t9350, t9367, t9368)
}
