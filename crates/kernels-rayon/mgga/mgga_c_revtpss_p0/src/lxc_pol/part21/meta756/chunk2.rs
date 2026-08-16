//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2653/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2653(t5618: f64, t9784: f64, t820: f64, t844: f64, t9991: f64, t13807: f64, t13767: f64, t2661: f64, t3829: f64, t48347: f64, t13776: f64, t9775: f64) -> (f64, f64, f64, f64) {
    let t48833 = t9784 * t5618;
    let t48836 = t820 * t9991 * t844;
    let t48837 = t48836 * t13807;
    let t48845 = t2661 * t13767 * t48347 * t3829;
    let t48847 = t9775 * t13776;
    (t48833, t48837, t48845, t48847)
}
