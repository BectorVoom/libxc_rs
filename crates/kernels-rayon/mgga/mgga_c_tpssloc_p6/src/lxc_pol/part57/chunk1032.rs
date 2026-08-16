//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1032/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1032(t22633: f64, t33272: f64, t90566: f64, t1985: f64, t214: f64, t225: f64, t29286: f64, t567: f64, t28205: f64, t31611: f64, t102466: f64, t114178: f64, t115540: f64, t122247: f64, t122251: f64, t127325: f64, t127328: f64, t1375: f64, t20044: f64, t26224: f64, t26477: f64, t27068: f64, t31653: f64, t3887: f64, t6460: f64, t6461: f64, t7728: f64, t7729: f64, t7749: f64, t7750: f64, t7925: f64, t7936: f64, t8636: f64, t8637: f64) -> f64 {
    let t128740 = t22633 * t90566 * t33272;
    let t128745 = t1985 * t214 * t29286 * t225 * t567;
    let t128758 = t1985 * t31611 * t28205;
    let t128761 = 4.0_f64 * t27068 * t7729 + 2.0_f64 * t1375 * t3887 * t8636 * t6460 + 4.0_f64 * t26477 * t7925 - 2.0_f64 * t27068 * t7750 + 0.3289868133696452873e-1_f64 * t128740 + 0.82246703342411321825e-2_f64 * t128745 + t127325 - 12.0_f64 * t26224 * t102466 * t7728 - t127328 - t20044 * t8637 - t114178 + 4.0_f64 * t1375 * t3887 * t7936 * t7749 - t31653 * t6461 - t115540 + 0.82246703342411321824e-2_f64 * t122247 - 0.82246703342411321825e-2_f64 * t128758 + 0.76763589786250567036e-1_f64 * t122251;
    t128761
}
