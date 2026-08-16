//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1074/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1074(t14792: f64, t885: f64, t11289: f64, t11366: f64, t1437: f64, t14658: f64, t14685: f64, t14734: f64, t14739: f64, t2545: f64, t2589: f64, t3822: f64, t3827: f64, t3845: f64, t4892: f64, t4908: f64, t4911: f64, t4940: f64, t4943: f64, t877: f64, t8842: f64, t886: f64, t8899: f64, t8912: f64, t896: f64) -> f64 {
    let t14793 = t14792 * t885;
    let t14800 = 0.5848223622634646207e0_f64 * t2589 * t4940 + 0.5848223622634646207e0_f64 * t896 * t14734 + 0.17315859105681463759e2_f64 * t8912 * t4943 + t14658 - t14685 + 1.0_f64 * t14739 * t886 + 2.0_f64 * t11289 * t1437 + 2.0_f64 * t3822 * t3845 - 2.0_f64 * t8899 * t4892 + 1.0_f64 * t2545 * t4908 + 1.0_f64 * t877 * t14793 + 0.32163958997385070134e2_f64 * t8842 * t4911 - 4.0_f64 * t11366 * t3827;
    t14800
}
