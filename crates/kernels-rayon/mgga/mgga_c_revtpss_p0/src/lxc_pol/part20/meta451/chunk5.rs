//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1722/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1722(t3970: f64, t9779: f64, t9765: f64, t9923: f64, t125: f64, t1399: f64, t1410: f64, t3934: f64, t3936: f64, t3938: f64, t4012: f64, t4057: f64, t46298: f64, t46655: f64, t46660: f64, t46671: f64, t46680: f64, t46682: f64, t46692: f64, t46695: f64, t46702: f64, t46704: f64, t5671: f64, t5673: f64, t828: f64, t9628: f64, t9810: f64, t9826: f64, t9835: f64, t9840: f64) -> f64 {
    let t46706 = t9779 * t3970;
    let t46712 = t9765 * t9923;
    let t46714 = 0.34299214494455789577e-2_f64 * t3934 * t3936 * t46655 * t3938 + 0.96037800584476210818e-1_f64 * t46660 - 0.20579528696673473747e-1_f64 * t5671 * t3936 * t46655 * t9835 + 0.77173232612525526552e-2_f64 * t5671 * t5673 * t9826 * t9840 - 0.73180804045370872643e-3_f64 * t46671 + 0.51448821741683684366e-2_f64 * t3934 * t3936 * t9826 * t9810 + 0.60984003371142393869e-3_f64 * t46680 + 0.34299214494455789577e-2_f64 * t3934 * t3936 * t46682 * t3938 - 0.12862205435420921092e-2_f64 * t3934 * t5673 * t9826 * t4057 + 7.0_f64 / 3.0_f64 * t46692 + 35.0_f64 / 12.0_f64 * t46695 + 0.34299214494455789577e-2_f64 * t3934 * t3936 * t125 * t9628 * t1399 + 0.45178982497454656791e-6_f64 * t46702 + 0.91464571985215438873e-3_f64 * t46704 - 0.13605355082800796532e0_f64 * t46706 + 0.12862205435420921092e-1_f64 * t1410 * t4012 * t828 * t46298 - 0.16262400898971305032e-1_f64 * t46712;
    t46714
}
