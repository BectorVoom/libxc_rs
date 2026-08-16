//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 734/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk734(t6579: f64, t7525: f64, t6547: f64, t7529: f64, t23168: f64, t7521: f64, t22893: f64, t7520: f64, t23164: f64, t1519: f64, t234: f64, t23204: f64, t7479: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25277 = t6579 * t7525;
    let t25293 = t6547 * t7529;
    let t25310 = t23168 * t7521;
    let t25316 = t22893 * t7520;
    let t25317 = t23164 * t25316;
    let t25319 = t234 * t1519;
    let t25345 = t23204 * t7479;
    (t25277, t25293, t25310, t25317, t25319, t25345)
}
