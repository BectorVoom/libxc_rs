//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 942/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk942(t4475: f64, t950: f64, t1569: f64, t1581: f64, t2856: f64, t2861: f64, t2886: f64, t2900: f64, t2905: f64, t2930: f64, t311: f64, t4353: f64, t4356: f64, t4358: f64, t4361: f64, t4398: f64, t4402: f64, t4408: f64, t4411: f64, t4416: f64, t4434: f64, t4438: f64, t4447: f64, t4449: f64, t4454: f64, t4472: f64, t924: f64, t933: f64, t943: f64, t952: f64) -> (f64, f64) {
    let t4476 = t4475 * t950;
    let t4479 = -0.310907e-1_f64 * t4408 * t311 + 1.0_f64 * t4411 * t933 + 1.0_f64 * t2856 * t1569 - 2.0_f64 * t2861 * t4416 + 1.0_f64 * t924 * t4434 + 0.32163958997385070134e2_f64 * t2886 * t4438 + t4353 - t4356 - t4358 + t4361 - t4398 - t4402 - 0.19751673498613801407e-1_f64 * t4447 + 0.5848223622634646207e0_f64 * t4449 * t952 + 0.5848223622634646207e0_f64 * t2900 * t1581 - 0.11696447245269292414e1_f64 * t2905 * t4454 + 0.5848223622634646207e0_f64 * t943 * t4472 + 0.17315859105681463759e2_f64 * t2930 * t4476;
    (t4476, t4479)
}
