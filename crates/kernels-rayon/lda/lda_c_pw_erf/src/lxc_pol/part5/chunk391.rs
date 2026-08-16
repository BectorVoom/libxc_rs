//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 391/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk391(t1765: f64, t402: f64, t19: f64, t729: f64, t887: f64, t1: f64, t748: f64, t397: f64, t390: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1766 = t1765 * t402;
    let t1767 = 0.5848223397455204_f64 * t1766;
    let t1769 = t887 * t729 * t19;
    let t1772 = t748 * t1;
    let t1773 = t1772 * t397;
    let t1774 = 0.0001831155503675316_f64 * t1773;
    let t1775 = t748 * t390;
    (t1766, t1767, t1769, t1772, t1773, t1774, t1775)
}
