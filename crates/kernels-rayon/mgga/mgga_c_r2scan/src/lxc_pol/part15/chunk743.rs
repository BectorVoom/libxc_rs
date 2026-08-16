//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 743/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk743(t6028: f64, t6029: f64, t1375: f64, t2060: f64, t2062: f64, t1783: f64, t759: f64, t761: f64, t2049: f64, t607: f64, t4733: f64, t4736: f64, t4739: f64, t5860: f64) -> (f64, f64, f64, f64, f64) {
    let t6030 = t6028 * t6029;
    let t6032 = t2060 * t1375;
    let t6033 = t6032 * t2062;
    let t6036 = t759 * t1783 * t761;
    let t6038 = t607 * t2049;
    let t6039 = t759 * t6038;
    let t6044 = -0.29633333333333333333e-1_f64 * t4733 + 0.19755555555555555555e-1_f64 * t4736 - 0.23048148148148148148e-1_f64 * t4739 - t5860;
    (t6030, t6033, t6036, t6039, t6044)
}
