//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1951/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1951(t9552: f64, t9559: f64, t1317: f64, t5567: f64, t9564: f64, t9566: f64, t9578: f64, t9580: f64, t4147: f64, t5778: f64, t2496: f64, t5571: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13640 = 0.5848223622634646207e0_f64 * t9552;
    let t13641 = 40.0_f64 * t9559;
    let t13643 = 8.0_f64 * t1317 * t5567;
    let t13644 = 0.18311447306006545054e-3_f64 * t9564;
    let t13645 = 0.4883052614935078681e-3_f64 * t9566;
    let t13646 = 24.0_f64 * t9578;
    let t13647 = 4.0_f64 * t9580;
    let t13648 = t5778 * t4147;
    let t13652 = t5571 * t2496;
    (t13640, t13641, t13643, t13644, t13645, t13646, t13647, t13648, t13652)
}
