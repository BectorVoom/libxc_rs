//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1443/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1443(t33281: f64, t6914: f64, t1351: f64, t1992: f64, t550: f64, t6976: f64, t7918: f64, t115391: f64, t120441: f64, t120445: f64, t120447: f64, t120452: f64, t120456: f64, t120459: f64, t120463: f64, t120467: f64, t122451: f64, t122457: f64, t122460: f64, t1336: f64, t1814: f64, t31636: f64, t31639: f64, t5287: f64) -> f64 {
    let t122462 = t6914 * t33281;
    let t122467 = t1992 * t6976 * t7918 * t1351 * t550;
    let t122470 = 0.49348022005446793095e-1_f64 * t122451 - t1336 * t31636 * t5287 + t120441 - t120445 + t120447 - t120452 - 0.82246703342411321825e-2_f64 * t122457 + 0.41123351671205660912e-2_f64 * t122460 - t120456 + t120459 + t120463 + t120467 + 0.19190897446562641759e-1_f64 * t122462 - 0.82246703342411321825e-2_f64 * t122467 - t115391 + t1814 * t31639;
    t122470
}
