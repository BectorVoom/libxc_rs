//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1162/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1162(t31399: f64, t858: f64, t23204: f64, t8547: f64, t6562: f64, t2053: f64, t2718: f64, t6662: f64, t26728: f64, t6631: f64, t6571: f64, t7106: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31400 = t858 * t31399;
    let t31405 = t23204 * t8547;
    let t31406 = t6562 * t31405;
    let t31407 = 0.41123351671205660912e-2_f64 * t31406;
    let t31409 = t2718 * t2053 * t6662;
    let t31416 = t26728 * t6631;
    let t31419 = t6571 * t7106;
    (t31400, t31405, t31407, t31409, t31416, t31419)
}
