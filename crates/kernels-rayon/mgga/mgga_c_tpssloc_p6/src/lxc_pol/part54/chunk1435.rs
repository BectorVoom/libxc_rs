//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1435/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1435(t33285: f64, t6883: f64, t33284: f64, t6897: f64, t794: f64, t1992: f64, t22897: f64, t27075: f64, t27078: f64, t6976: f64, t115430: f64, t115433: f64, t115435: f64, t115439: f64, t120483: f64, t120487: f64, t120491: f64, t120496: f64, t120502: f64, t122439: f64, t122471: f64, t1332: f64, t1352: f64, t33291: f64, t5344: f64, t544: f64, t553: f64) -> f64 {
    let t122503 = t6883 * t33285;
    let t122507 = t6897 * t794 * t33284;
    let t122510 = t1992 * t22897 * t27075;
    let t122513 = t1992 * t6976 * t27078;
    let t122515 = -0.19190897446562641759e-1_f64 * t115430 + t115433 + t115435 - t5344 * t122471 * t1352 + t544 * t553 * t122439 - t120483 - t120487 + t120491 - 0.41123351671205660912e-2_f64 * t115439 - 0.19190897446562641759e-1_f64 * t122503 - t120496 + t1332 * t33291 - 0.41123351671205660912e-2_f64 * t122507 + 0.16449340668482264365e-1_f64 * t122510 - 0.82246703342411321825e-2_f64 * t122513 - t120502;
    t122515
}
