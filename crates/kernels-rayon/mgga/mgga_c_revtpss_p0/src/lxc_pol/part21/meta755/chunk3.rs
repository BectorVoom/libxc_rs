//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2650/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2650(t2661: f64, t3992: f64, t4057: f64, t5608: f64, t4004: f64, t5651: f64, t9934: f64, t47198: f64, t5665: f64, t5629: f64, t9779: f64, t5661: f64, t9909: f64) -> (f64, f64, f64, f64, f64) {
    let t48786 = t2661 * t3992 * t5608 * t4057;
    let t48790 = t2661 * t9934 * t5651 * t4004;
    let t48792 = t47198 * t5665;
    let t48794 = t9779 * t5629;
    let t48796 = t9909 * t5661;
    (t48786, t48790, t48792, t48794, t48796)
}
