//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 655/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk655<F: Float>(t1932: F, t360: F, t3187: F, t3166: F, t383: F, t1003: F, t1058: F, t1061: F, t1063: F, t3076: F, t3180: F, t3186: F, t3189: F, t3193: F, t3197: F, t3200: F, t353: F, t384: F) -> (F, F, F, F) {
    let t3201 = t1932 * t360;
    let t3202 = t3187 * t3201;
    let t3204 = t383 * t3166;
    let t3206 = F::cast_from(2.0_f64) * t1003 * t1063 + F::cast_from(2.0_f64) * t1058 * t3193 + t1058 * t3197 + F::cast_from(2.0_f64) * t1061 * t3180 + t3076 * t384 + F::cast_from(2.0_f64) * t3186 * t3189 - t3200 * t3202 + t3204 * t353;
    (t3201, t3202, t3204, t3206)
}
