//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1410/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1410(t40690: f64, t5610: f64, t5618: f64, t9784: f64, t46644: f64, t5622: f64, t40488: f64, t40763: f64, t5609: f64, t9793: f64, t268: f64, t5617: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t48829 = t40690 * t5610;
    let t48833 = t9784 * t5618;
    let t48849 = t46644 * t5622;
    let t48853 = t40488 * t5610;
    let t48879 = t9793 * t40763 * t5609;
    let t48908 = t5617 * t268;
    (t48829, t48833, t48849, t48853, t48879, t48908)
}
