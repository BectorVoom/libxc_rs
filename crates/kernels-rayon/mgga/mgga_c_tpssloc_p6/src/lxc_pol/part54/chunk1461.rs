//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1461/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1461(t1437: f64, t31860: f64, t32343: f64, t8513: f64, t117480: f64, t1433: f64, t8663: f64, t63: f64, t641: f64, t116082: f64, t116124: f64, t117483: f64, t117499: f64, t117516: f64, t117518: f64, t117527: f64, t122960: f64, t122964: f64, t122988: f64, t123001: f64, t31857: f64, t31868: f64, t32328: f64, t32338: f64, t32340: f64, t33669: f64, t33677: f64, t34122: f64, t34132: f64, t4017: f64, t4021: f64, t8824: f64, t8825: f64) -> f64 {
    let t124834 = t31860 * t8513 * t32343 * t1437;
    let t124838 = t8663 * t8513 * t117480 * t1433;
    let t124844 = t641 * t63;
    let t124860 = 10.0_f64 / 27.0_f64 * t117483 + 5.0_f64 / 12.0_f64 * t122988 * t32328 - 5.0_f64 / 36.0_f64 * t33669 * t32340 + 5.0_f64 / 12.0_f64 * t116124 * t34122 + 5.0_f64 / 12.0_f64 * t116082 * t34122 + 5.0_f64 / 12.0_f64 * t31860 * t8513 * t8824 * t4021 + 5.0_f64 / 12.0_f64 * t123001 * t32328 - 5.0_f64 / 36.0_f64 * t33677 * t32340 - 10.0_f64 / 9.0_f64 * t124834 + 10.0_f64 / 27.0_f64 * t124838 - 5.0_f64 / 36.0_f64 * t31857 * t34132 - 5.0_f64 / 36.0_f64 * t31868 * t34132 - 5.0_f64 / 36.0_f64 * t8663 * t8513 * t124844 * t1433 - 5.0_f64 / 36.0_f64 * t8663 * t8513 * t32338 * t4017 - 20.0_f64 / 27.0_f64 * t117499 + 5.0_f64 / 27.0_f64 * t117516 + 5.0_f64 / 27.0_f64 * t117518 - t117527 - 5.0_f64 / 72.0_f64 * t122960 * t8825 - 5.0_f64 / 72.0_f64 * t122964 * t8825;
    t124860
}
