//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 731/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk731(t423: f64, t6027: f64, t2056: f64, t597: f64, t2049: f64, t607: f64, t759: f64, t4733: f64, t4736: f64, t4739: f64, t5860: f64, t166: f64) -> (f64, f64, f64, f64, f64) {
    let t6028 = t6027 * t423;
    let t6029 = t597 * t2056;
    let t6030 = t6028 * t6029;
    let t6038 = t607 * t2049;
    let t6039 = t759 * t6038;
    let t6044 = -0.29633333333333333333e-1_f64 * t4733 + 0.19755555555555555555e-1_f64 * t4736 - 0.23048148148148148148e-1_f64 * t4739 - t5860;
    let t6045 = t166 * t6044;
    (t6029, t6030, t6039, t6044, t6045)
}
