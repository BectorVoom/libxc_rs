//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1116/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1116<F: Float>(t233: F, t29232: F, t1658: F, t8121: F, t2167: F, t6290: F, t10819: F, t1259: F, t3530: F, t3622: F, t779: F, t9274: F) -> (F, F, F, F, F, F, F) {
    let t29233 = t233 * t29232;
    let t29234 = t29233 / F::cast_from(16.0_f64);
    let t29235 = t1658 * t8121;
    let t29236 = t233 * t29235;
    let t29237 = t29236 / F::cast_from(8.0_f64);
    let t29238 = t6290 * t2167;
    let t30045 = t1259 * t10819;
    let t30066 = t3530 * t3622;
    let t31271 = t779 * t9274;
    (t29234, t29235, t29237, t29238, t30045, t30066, t31271)
}
