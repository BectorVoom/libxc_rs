//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 745/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk745(t4939: f64, t904: f64, t2621: f64, t4923: f64, t1437: f64, t1449: f64, t2550: f64, t2575: f64, t2594: f64, t2619: f64, t305: f64, t3822: f64, t3860: f64, t4840: f64, t4842: f64, t4846: f64, t4878: f64, t4881: f64, t4886: f64, t4892: f64, t4908: f64, t4911: f64, t4919: f64, t4924: f64, t877: f64, t896: f64) -> (f64, f64, f64) {
    let t4940 = t4939 * t904;
    let t4943 = t4923 * t2621;
    let t4946 = -0.310907e-1_f64 * t4886 * t305 + 2.0_f64 * t3822 * t1437 - 2.0_f64 * t2550 * t4892 + 1.0_f64 * t877 * t4908 + 0.32163958997385070134e2_f64 * t2575 * t4911 + t4840 - t4842 + t4846 - t4878 - t4881 - 0.19751673498613801407e-1_f64 * t4919 + 0.11696447245269292414e1_f64 * t3860 * t1449 - 0.11696447245269292414e1_f64 * t2594 * t4924 + 0.5848223622634646207e0_f64 * t896 * t4940 + 0.17315859105681463759e2_f64 * t2619 * t4943;
    (t4940, t4943, t4946)
}
