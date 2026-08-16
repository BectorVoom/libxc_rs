//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2763/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2763(t22020: f64, t2661: f64, t5675: f64, t9934: f64, t22267: f64, t9976: f64, t13847: f64, t1399: f64, t73731: f64, t9816: f64, t22294: f64, t48862: f64, t48999: f64) -> (f64, f64, f64, f64) {
    let t73951 = t2661 * t9934 * t22020 * t5675;
    let t73953 = t9976 * t22267;
    let t73963 = t9816 * t13847 * t73731 * t1399;
    let t73975 = t48862 * t48999 * t22294;
    (t73951, t73953, t73963, t73975)
}
