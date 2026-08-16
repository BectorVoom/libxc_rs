//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1209/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1209(t12657: f64, t487: f64, t1210: f64, t1215: f64, t12600: f64, t12603: f64, t12607: f64, t12622: f64, t12628: f64, t12630: f64, t12633: f64, t12641: f64, t12647: f64, t12651: f64, t12654: f64, t1295: f64, t3556: f64, t3567: f64, t3569: f64, t3572: f64, t3576: f64, t3585: f64, t3732: f64, t3791: f64) -> (f64, f64) {
    let t12658 = t12657 * t487;
    let t12663 = -0.39512695097613069591e1_f64 * t3567 * t12600 - 0.39512695097613069591e1_f64 * t12603 * t1295 + 0.19756347548806534796e1_f64 * t1210 * t12607 - 0.65854491829355115987e0_f64 * t1210 * t12622 - 0.39512695097613069591e1_f64 * t12628 * t12630 + 0.39512695097613069591e1_f64 * t12633 * t3569 + 0.39512695097613069591e1_f64 * t3572 * t3576 - 0.19756347548806534796e1_f64 * t3572 * t3585 + 0.39512695097613069591e1_f64 * t12641 * t3569 + 0.39512695097613069591e1_f64 * t3556 * t3576 + 0.39512695097613069591e1_f64 * t3567 * t12647 + 0.19756347548806534796e1_f64 * t1210 * t12651 - 0.19756347548806534796e1_f64 * t12654 * t1295 - 0.19756347548806534796e1_f64 * t12658 * t1215 - 0.19756347548806534796e1_f64 * t3732 * t3791;
    (t12658, t12663)
}
