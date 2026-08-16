//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2555/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2555(t11356: f64, t11366: f64, t11434: f64, t1148: f64, t1156: f64, t15133: f64, t3334: f64, t3371: f64, t3378: f64, t436: f64, t44211: f64, t4802: f64, t4858: f64, t51107: f64, t51669: f64, t51677: f64, t51680: f64, t51725: f64, t51727: f64, t51730: f64, t51736: f64, t51738: f64, t51741: f64, t51744: f64, t51765: f64, t51785: f64) -> f64 {
    let t51789 = -6.0_f64 * t44211 * t4802 - t51669 + 0.17544670867903938621e1_f64 * t11356 * t4858 + 0.17544670867903938621e1_f64 * t3371 * t15133 + 0.5848223622634646207e0_f64 * t1148 * t51107 * t1156 - 0.35089341735807877242e1_f64 * t51677 * t3378 - 0.10389515463408878255e3_f64 * t51680 * t11366 - 0.19751673498613801407e-1_f64 * t51725 - 0.31168546390226634766e3_f64 * t51727 * t11434 - 6.0_f64 * t51730 * t3334 - t51736 - t51738 - t51741 - t51744 - 0.310907e-1_f64 * (t51765 + t51785) * t436;
    t51789
}
