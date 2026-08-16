//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2170/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2170(t11262: f64, t1670: f64, t1041: f64, t3172: f64, t4824: f64, t3127: f64, t3211: f64, t4845: f64, t1053: f64, t4857: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15731 = t11262 * t1670;
    let t15732 = t1041 * t15731;
    let t15734 = t3172 * t4824;
    let t15736 = 0.19055119163586549765e-3_f64 * t3127 * t15734;
    let t15744 = 0.15244095330869239812e-2_f64 * t3211 * t4845;
    let t15745 = t4857 * t1053;
    (t15731, t15732, t15734, t15736, t15744, t15745)
}
