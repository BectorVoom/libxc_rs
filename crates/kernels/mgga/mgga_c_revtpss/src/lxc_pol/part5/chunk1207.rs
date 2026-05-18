//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1207/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1207<F: Float>(t19021: F, t973: F, t11461: F, t11554: F, t15343: F, t1634: F, t19029: F, t19031: F, t19058: F, t19060: F, t19062: F, t19156: F, t2982: F, t4685: F, t4708: F, t6190: F, t6206: F, t6209: F, t965: F, t974: F) -> F {
    let t19167 = t19021 * t973;
    let t19172 = t19029 - t19031 + F::new(0.5848223622634646207e0) * t19156 * t974 + F::new(0.11696447245269292414e1) * t15343 * t1634 + F::new(0.11696447245269292414e1) * t4685 * t4708 - F::new(0.11696447245269292414e1) * t11554 * t6190 + F::new(0.5848223622634646207e0) * t2982 * t6206 + F::new(0.5848223622634646207e0) * t965 * t19167 + F::new(0.17315859105681463759e2) * t11461 * t6209 - t19058 - t19060 - t19062;
    t19172
}
