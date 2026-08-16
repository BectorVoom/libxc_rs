//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1930/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1930(t4004: f64, t5673: f64, t5674: f64, t9840: f64, t1868: f64, t3829: f64, t828: f64, t9942: f64, t5608: f64, t5675: f64, t9934: f64, t2661: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13817 = t5673 * t5674 * t4004;
    let t13821 = t5673 * t5674 * t9840;
    let t13824 = t1868 * t3829;
    let t13826 = t9942 * t828 * t13824;
    let t13829 = t5608 * t5675;
    let t13830 = t9934 * t13829;
    let t13832 = 0.28582678745379824648e-4_f64 * t2661 * t13830;
    (t13817, t13821, t13824, t13826, t13830, t13832)
}
