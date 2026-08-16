//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1436/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1436(t1760: f64, t5392: f64, t103130: f64, t103143: f64, t103149: f64, t103188: f64, t103314: f64, t1238: f64, t1653: f64, t1716: f64, t22004: f64, t22394: f64, t24589: f64, t24601: f64, t27406: f64, t27820: f64, t29536: f64, t29794: f64, t29804: f64, t29808: f64, t29816: f64, t3598: f64, t5055: f64, t6146: f64, t7283: f64, t7351: f64, t8002: f64, t8010: f64, t85652: f64, t94458: f64, t94514: f64) -> (f64, f64) {
    let t109060 = t5392 * t1760;
    let t109096 = 0.82246703342411321826e-2_f64 * t24589 * t24601 * t103314 * t1653 + 0.16449340668482264365e-1_f64 * t24589 * t24601 * t85652 * t109060 + 0.16449340668482264365e-1_f64 * t24589 * t27820 * t29816 + 0.54831135561607547883e-2_f64 * t103130 + 0.16449340668482264365e-1_f64 * t24589 * t94458 * t29808 + 0.16449340668482264365e-1_f64 * t24589 * t103143 * t8002 + 0.54831135561607547883e-2_f64 * t103149 - 0.13159472534785811492e0_f64 * t27406 * t29804 + 0.49348022005446793095e-1_f64 * t7283 * t1716 * t103188 - 0.16449340668482264365e-1_f64 * t24589 * t94514 * t29808 - 0.24674011002723396548e-1_f64 * t7283 * t6146 * t8010 + 6.0_f64 * t7351 * t22004 + 6.0_f64 * t1238 * t3598 * t29794 * t1760 - t7351 * t22394 + 6.0_f64 * t5055 * t29536;
    (t109060, t109096)
}
