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
    let t11202 = F::new(3.0) / F::new(2.0) * t11201;
    let t11204 = t3275 * t11199 * t3352;
    let t11205 = t11204 / F::new(2.0);
    let t11206 = F::new(22.0) / F::new(9.0) * t11031;
    let t11215 = F::new(22.0) / F::new(9.0) * t11057;
    let t11216 = -t11206 - F::new(4.0) / F::new(3.0) * t11034 - t11037 / F::new(2.0) + t11039 / F::new(4.0) - t11041 / F::new(4.0) + t11043 + F::new(4.0) / F::new(3.0) * t11045 - F::new(3.0) / F::new(2.0) * t11048 - F::new(8.0) / F::new(3.0) * t11051 + t11054 / F::new(2.0) - t11215;
    (t11202, t11205, t11206, t11215, t11216)
}
