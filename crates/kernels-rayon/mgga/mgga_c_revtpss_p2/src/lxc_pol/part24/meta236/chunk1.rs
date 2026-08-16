//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 997/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk997(t13955: f64, t9845: f64, t1885: f64, t9909: f64, t4000: f64, t820: f64, t844: f64, t2713: f64, t3964: f64, t5617: f64, t5665: f64, t9976: f64) -> (f64, f64, f64, f64, f64) {
    let t13956 = t9845 * t13955;
    let t13959 = t9909 * t1885;
    let t13999 = t820 * t4000 * t844;
    let t14013 = t3964 * t2713 * t5617;
    let t14043 = t9976 * t5665;
    (t13956, t13959, t13999, t14013, t14043)
}
