//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1251/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1251(t19021: f64, t973: f64, t11461: f64, t11554: f64, t15343: f64, t1634: f64, t19029: f64, t19031: f64, t19058: f64, t19060: f64, t19062: f64, t19156: f64, t2982: f64, t4685: f64, t4708: f64, t6190: f64, t6206: f64, t6209: f64, t965: f64, t974: f64) -> f64 {
    let t19167 = t19021 * t973;
    let t19172 = t19029 - t19031 + 0.5848223622634646207e0_f64 * t19156 * t974 + 0.11696447245269292414e1_f64 * t15343 * t1634 + 0.11696447245269292414e1_f64 * t4685 * t4708 - 0.11696447245269292414e1_f64 * t11554 * t6190 + 0.5848223622634646207e0_f64 * t2982 * t6206 + 0.5848223622634646207e0_f64 * t965 * t19167 + 0.17315859105681463759e2_f64 * t11461 * t6209 - t19058 - t19060 - t19062;
    t19172
}
