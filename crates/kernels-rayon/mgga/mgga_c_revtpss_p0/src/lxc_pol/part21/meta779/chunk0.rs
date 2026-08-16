//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2774/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2774(t14767: f64, t221: f64, t10703: f64, t2674: f64, t2661: f64, t2662: f64, t2754: f64, t4352: f64, t14728: f64, t9775: f64, t1549: f64, t40861: f64) -> (f64, f64, f64, f64) {
    let t50931 = t221 * t14767;
    let t50933 = t2674 * t10703 * t50931;
    let t50937 = t2661 * t2662 * t4352 * t2754;
    let t50939 = t9775 * t14728;
    let t50941 = t40861 * t1549;
    (t50933, t50937, t50939, t50941)
}
