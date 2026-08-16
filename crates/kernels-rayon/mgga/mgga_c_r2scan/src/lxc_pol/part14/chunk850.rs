//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 850/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk850(t1831: f64, t963: f64, t2747: f64, t750: f64, t1842: f64, t1814: f64, t5249: f64, t897: f64, t5252: f64, t5298: f64, t5302: f64, t5303: f64, t5307: f64, t5321: f64, t5323: f64, t5327: f64) -> f64 {
    let t7685 = t963 * t1831;
    let t7688 = 0.34631718211362927518e2_f64 * t2747 * t750;
    let t7689 = t963 * t1842;
    let t7691 = t963 * t1814;
    let t7693 = t5249 * t897;
    let t7694 = t7693 * t5252;
    let t7696 = t5298 + t5302 + 0.34631718211362927518e2_f64 * t5303 + t5307 + t5321 + 0.2701041328e0_f64 * t5323 + 0.2701041328e0_f64 * t5327 - 0.11696447245269292414e1_f64 * t7685 + t7688 + 0.34631718211362927518e2_f64 * t7689 + 0.17315859105681463759e2_f64 * t7691 - 0.4051561992e0_f64 * t7694;
    t7696
}
