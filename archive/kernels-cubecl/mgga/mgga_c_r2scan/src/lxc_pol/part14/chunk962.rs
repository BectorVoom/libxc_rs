//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 962/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk962<F: Float>(t11199: F, t3262: F, t3264: F, t3275: F, t3352: F, t11031: F, t11057: F, t11034: F, t11037: F, t11039: F, t11041: F, t11043: F, t11045: F, t11048: F, t11051: F, t11054: F) -> (F, F, F, F, F) {
    let t11201 = t3262 * t11199 * t3264;
    let t11202 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t11201;
    let t11204 = t3275 * t11199 * t3352;
    let t11205 = t11204 / F::cast_from(2.0_f64);
    let t11206 = F::cast_from(22.0_f64) / F::cast_from(9.0_f64) * t11031;
    let t11215 = F::cast_from(22.0_f64) / F::cast_from(9.0_f64) * t11057;
    let t11216 = -t11206 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t11034 - t11037 / F::cast_from(2.0_f64) + t11039 / F::cast_from(4.0_f64) - t11041 / F::cast_from(4.0_f64) + t11043 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t11045 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t11048 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t11051 + t11054 / F::cast_from(2.0_f64) - t11215;
    (t11202, t11205, t11206, t11215, t11216)
}
