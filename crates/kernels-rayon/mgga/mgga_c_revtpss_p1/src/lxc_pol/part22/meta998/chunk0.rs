//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3389/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3389(t11294: f64, t19331: f64, t19127: f64, t2926: f64, t2924: f64, t934: f64, t3007: f64, t6226: f64, t981: f64, t4631: f64, t15543: f64, t4719: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t63649 = 0.32163958997385070134e2_f64 * t11294 * t19331;
    let t63650 = t19127 * t2926;
    let t63653 = 0.32163958997385070134e2_f64 * t2924 * t63650 * t934;
    let t63656 = 0.35089341735807877242e1_f64 * t981 * t6226 * t3007;
    let t63657 = t4631 * t4631;
    let t63660 = 0.32163958997385070134e2_f64 * t2924 * t63657 * t2926;
    let t63662 = 0.20508037716432813315e4_f64 * t4719 * t15543;
    (t63649, t63653, t63656, t63657, t63660, t63662)
}
