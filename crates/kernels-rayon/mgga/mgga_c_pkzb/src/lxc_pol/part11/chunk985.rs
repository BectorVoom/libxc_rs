//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 985/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk985(t10769: f64, t5758: f64, t7357: f64, t9148: f64, t261: f64, t5745: f64, t228: f64, t1084: f64, t3550: f64, t1855: f64, t1083: f64, t9228: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10771 = -t5758 + 0.37083333333333333334e-1_f64 * t7357 - 0.278125e-1_f64 * t9148 + 0.278125e-1_f64 * t10769;
    let t10772 = t10771 * t261;
    let t10777 = -t5745 + 0.71233333333333333332e-1_f64 * t7357 - 0.53424999999999999999e-1_f64 * t9148 + 0.53425e-1_f64 * t10769;
    let t10779 = 0.621814e-1_f64 * t10777 * t228;
    let t10780 = t1084 * t3550;
    let t10782 = 6.0_f64 * t1855 * t10780;
    let t10783 = t9228 * t1083;
    (t10771, t10772, t10777, t10779, t10780, t10782, t10783)
}
