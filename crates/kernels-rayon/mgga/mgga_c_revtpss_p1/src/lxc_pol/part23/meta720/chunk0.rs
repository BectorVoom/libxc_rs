//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2480/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2480(t48614: f64, t14005: f64, t46740: f64, t46917: f64, t5697: f64, t14036: f64, t9976: f64, t46694: f64, t5686: f64, t13769: f64, t808: f64, t9736: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t48615 = 0.17006693853500995666e-1_f64 * t48614;
    let t48637 = t46740 * t14005;
    let t48638 = 0.40656002247428262579e-3_f64 * t48637;
    let t48645 = t46917 * t5697;
    let t48668 = t9976 * t14036;
    let t48669 = 0.40656002247428262579e-3_f64 * t48668;
    let t48685 = t46694 * t5686;
    let t48686 = 35.0_f64 / 24.0_f64 * t48685;
    let t48690 = t9736 * t808 * t13769;
    (t48615, t48638, t48645, t48669, t48686, t48690)
}
