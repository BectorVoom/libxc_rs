//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 803/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk803<F: Float>(t3005: F, t486: F, t1767: F, t206: F, t4068: F, t4077: F, t591: F, t4080: F, t4111: F, t1180: F, t209: F, t211: F, t4094: F, t4096: F, t4103: F, t574: F) -> (F, F, F, F, F, F, F, F) {
    let t9352 = t486 * t3005;
    let t9408 = 0.008082336938271605 * t206 * t1767 * t4068;
    let t9410 = 8.0 / 9.0 * t4077 * t591;
    let t9412 = 2e-21 * t4080 * t4111;
    let t9417 = 56.0 / 243.0 * t209 * t211 * t1180;
    let t9422 = t4094 * t591;
    let t9424 = t4096 * t4111;
    let t9426 = t574 * t4103;
    (t9352, t9408, t9410, t9412, t9417, t9422, t9424, t9426)
}
