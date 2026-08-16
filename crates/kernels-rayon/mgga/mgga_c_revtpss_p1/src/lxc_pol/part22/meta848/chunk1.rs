//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2988/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2988(t2782: f64, t4077: f64, t47794: f64, t556: f64, t1426: f64, t5711: f64, t786: f64, t3917: f64, t3899: f64, t5775: f64, t689: f64, t14100: f64, t9686: f64) -> (f64, f64, f64, f64, f64) {
    let t49497 = t2782 * t556 * t47794 * t4077;
    let t49503 = t786 * t5711 * t1426;
    let t49504 = t49503 * t3917;
    let t49508 = t689 * t3899 * t5775;
    let t49512 = t14100 * t9686;
    (t49497, t49503, t49504, t49508, t49512)
}
