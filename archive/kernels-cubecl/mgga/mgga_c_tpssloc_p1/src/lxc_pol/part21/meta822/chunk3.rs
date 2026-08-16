//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2892/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2892<F: Float>(t10820: F, t14344: F, t17355: F, t17366: F, t2900: F, t2925: F, t2933: F, t42020: F, t42123: F, t4449: F, t5762: F, t5775: F, t5791: F, t60033: F, t60035: F, t60037: F, t60039: F, t60041: F, t60044: F, t60047: F, t60050: F, t60053: F, t60056: F, t60332: F, t60338: F, t60343: F, t943: F, t951: F, t952: F) -> F {
    let t60346 = -t60033 + t60035 + t60037 - t60039 - t60041 - t60044 - t60047 + t60050 + t60053 + t60056 + F::cast_from(0.11696447245269292414e1_f64) * t4449 * t14344 - F::cast_from(0.11696447245269292414e1_f64) * t42020 * t5775 + F::cast_from(0.5848223622634646207e0_f64) * t10820 * t5791 + F::cast_from(0.11696447245269292414e1_f64) * t2900 * t17366 + F::cast_from(0.5848223622634646207e0_f64) * t943 * t60332 * t951 + F::cast_from(0.32163958997385070134e2_f64) * t42123 * t5762 + F::cast_from(0.11696447245269292414e1_f64) * t60338 * t952 + F::cast_from(0.5848223622634646207e0_f64) * t17355 * t2925 + F::cast_from(0.17315859105681463759e2_f64) * t60343 * t2933;
    t60346
}
