//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2385/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2385(t159: f64, t216: f64, t2475: f64, t123: f64, t212: f64, t9291: f64, t2786: f64, t10914: f64, t2710: f64, t9285: f64, t2790: f64, t9292: f64) -> (f64, f64, f64, f64, f64) {
    let t40868 = t216 * t159 * t2475;
    let t40921 = t123 * t9291 * t212;
    let t40922 = t40921 * t2786;
    let t40945 = t2710 * t10914 * t9285;
    let t40958 = t9292 * t2790;
    (t40868, t40921, t40922, t40945, t40958)
}
