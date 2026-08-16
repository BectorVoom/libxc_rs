//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1382/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1382(t33383: f64, t6562: f64, t794: f64, t234: f64, t7823: f64, t6552: f64, t6637: f64, t776: f64, t118677: f64, t118679: f64, t118682: f64, t118694: f64, t118695: f64, t118699: f64, t118700: f64, t118710: f64, t118715: f64, t118719: f64, t121488: f64, t121493: f64, t121498: f64, t121501: f64, t812: f64, t829: f64) -> f64 {
    let t121504 = t6562 * t794 * t33383;
    let t121506 = t234 * t7823;
    let t121509 = t6552 * t6637 * t121506 * t776;
    let t121511 = t118677 + t118679 + t118682 + t118694 + t118695 + t118699 - t812 * t121488 * t829 + t118700 + 0.16449340668482264365e-1_f64 * t121493 + 0.16449340668482264365e-1_f64 * t121498 + 0.82246703342411321825e-2_f64 * t121501 - 0.41123351671205660912e-2_f64 * t121504 - 0.16449340668482264365e-1_f64 * t121509 - t118710 - t118715 + t118719;
    t121511
}
