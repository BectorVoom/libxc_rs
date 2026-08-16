//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1050/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1050(t785: f64, t860: f64, t780: f64, t2439: f64, t781: f64, t9292: f64, t861: f64, t867: f64, t786: f64, t2410: f64, t261: f64, t262: f64, t775: f64) -> (f64, f64, f64, f64, f64) {
    let t11028 = t785 * t860;
    let t11029 = t11028 * t780;
    let t11030 = t2439 * t11029;
    let t11040 = 0.17073386770573548589e-1_f64 * t9292 * t781;
    let t11043 = t861 * t867;
    let t11044 = t786 * t11043;
    let t11064 = 1.0_f64 / t2410 / t261;
    let t11088 = t262 * t775;
    (t11030, t11040, t11044, t11064, t11088)
}
