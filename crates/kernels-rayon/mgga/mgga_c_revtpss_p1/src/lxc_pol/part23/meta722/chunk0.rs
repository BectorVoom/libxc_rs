//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2484/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2484(t5618: f64, t9784: f64, t820: f64, t844: f64, t9991: f64, t13776: f64, t9775: f64, t46644: f64, t5622: f64, t5614: f64, t9779: f64, t40488: f64, t5610: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t48833 = t9784 * t5618;
    let t48836 = t820 * t9991 * t844;
    let t48847 = t9775 * t13776;
    let t48848 = 0.22866142996303859718e-3_f64 * t48847;
    let t48849 = t46644 * t5622;
    let t48851 = t9779 * t5614;
    let t48853 = t40488 * t5610;
    (t48833, t48836, t48848, t48849, t48851, t48853)
}
