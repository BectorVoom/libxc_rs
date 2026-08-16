//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 790/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk790(t13857: f64, t9793: f64, t2619: f64, t5635: f64, t2689: f64, t5618: f64, t5609: f64, t808: f64, t9845: f64, t1885: f64, t9909: f64, t2713: f64, t3964: f64, t5617: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13858 = t9793 * t13857;
    let t13887 = t5635 * t2619;
    let t13949 = t2689 * t5618;
    let t13955 = t808 * t5609;
    let t13956 = t9845 * t13955;
    let t13959 = t9909 * t1885;
    let t14013 = t3964 * t2713 * t5617;
    (t13858, t13887, t13949, t13956, t13959, t14013)
}
