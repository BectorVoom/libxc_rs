//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2539/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2539(t52037: f64, t52126: f64, t3011: f64, t4682: f64, t11506: f64, t1626: f64, t1609: f64, t2924: f64, t51973: f64, t52035: f64, t2942: f64, t4644: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t52598 = 0.45908888888888888888e0_f64 * t52037;
    let t52623 = 0.34731666666666666667e0_f64 * t52126;
    let t52637 = t4682 * t3011;
    let t52642 = t1626 * t11506;
    let t52645 = t2924 * t1609;
    let t52701 = 0.39862222222222222223e0_f64 * t51973;
    let t52751 = 0.27385555555555555556e0_f64 * t52126;
    let t52774 = 0.23744444444444444444e-1_f64 * t51973;
    let t52783 = 0.47488888888888888888e-1_f64 * t52035;
    let t52784 = 0.15829629629629629629e-1_f64 * t52037;
    let t52809 = t4644 * t2942;
    (t52598, t52623, t52637, t52642, t52645, t52701, t52751, t52774, t52783, t52784, t52809)
}
