//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 424/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk424(t1787: f64, t750: f64, t17: f64, t1804: f64, t3726: f64, t118: f64, t1799: f64, t794: f64, t3739: f64, t1808: f64, t225: f64) -> (f64, f64, f64, f64) {
    let t5168 = t1787 * t750;
    let t5169 = t17 * t5168;
    let t5192 = t3726 * t1804;
    let t5202 = t118 * t794 * t1799;
    let t5203 = t3739 * t5202;
    let t5215 = t1808 * t225;
    (t5169, t5192, t5203, t5215)
}
