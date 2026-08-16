//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1025/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1025(t7528: f64, t794: f64, t6562: f64, t6579: f64, t7525: f64, t6547: f64, t7529: f64, t23168: f64, t7521: f64, t22893: f64, t7520: f64, t23164: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25258 = t794 * t7528;
    let t25259 = t6562 * t25258;
    let t25277 = t6579 * t7525;
    let t25293 = t6547 * t7529;
    let t25310 = t23168 * t7521;
    let t25316 = t22893 * t7520;
    let t25317 = t23164 * t25316;
    (t25258, t25259, t25277, t25293, t25310, t25316, t25317)
}
