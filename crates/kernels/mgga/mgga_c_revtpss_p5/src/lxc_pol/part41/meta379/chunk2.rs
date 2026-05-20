//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1251/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1251<F: Float>(t19021: F, t973: F, t11461: F, t11554: F, t15343: F, t1634: F, t19029: F, t19031: F, t19058: F, t19060: F, t19062: F, t19156: F, t2982: F, t4685: F, t4708: F, t6190: F, t6206: F, t6209: F, t965: F, t974: F) -> F {
    let t19167 = t19021 * t973;
    let t19172 = t19029 - t19031 + F::cast_from(0.5848223622634646207e0_f64) * t19156 * t974 + F::cast_from(0.11696447245269292414e1_f64) * t15343 * t1634 + F::cast_from(0.11696447245269292414e1_f64) * t4685 * t4708 - F::cast_from(0.11696447245269292414e1_f64) * t11554 * t6190 + F::cast_from(0.5848223622634646207e0_f64) * t2982 * t6206 + F::cast_from(0.5848223622634646207e0_f64) * t965 * t19167 + F::cast_from(0.17315859105681463759e2_f64) * t11461 * t6209 - t19058 - t19060 - t19062;
    t19172
}
