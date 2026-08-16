//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 668/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk668<F: Float>(t4471: F, t951: F, t1580: F, t2932: F, t950: F, t1569: F, t1581: F, t2856: F, t2861: F, t2886: F, t2900: F, t2905: F, t2930: F, t311: F, t4353: F, t4356: F, t4358: F, t4361: F, t4398: F, t4402: F, t4408: F, t4411: F, t4416: F, t4434: F, t4438: F, t4447: F, t4449: F, t4454: F, t924: F, t933: F, t943: F, t952: F) -> (F, F, F, F) {
    let t4472 = t4471 * t951;
    let t4475 = t1580 * t2932;
    let t4476 = t4475 * t950;
    let t4479 = -F::cast_from(0.310907e-1_f64) * t4408 * t311 + F::cast_from(1.0_f64) * t4411 * t933 + F::cast_from(1.0_f64) * t2856 * t1569 - F::cast_from(2.0_f64) * t2861 * t4416 + F::cast_from(1.0_f64) * t924 * t4434 + F::cast_from(0.32163958997385070134e2_f64) * t2886 * t4438 + t4353 - t4356 - t4358 + t4361 - t4398 - t4402 - F::cast_from(0.19751673498613801407e-1_f64) * t4447 + F::cast_from(0.5848223622634646207e0_f64) * t4449 * t952 + F::cast_from(0.5848223622634646207e0_f64) * t2900 * t1581 - F::cast_from(0.11696447245269292414e1_f64) * t2905 * t4454 + F::cast_from(0.5848223622634646207e0_f64) * t943 * t4472 + F::cast_from(0.17315859105681463759e2_f64) * t2930 * t4476;
    (t4472, t4475, t4476, t4479)
}
