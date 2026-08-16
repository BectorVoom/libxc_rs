//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1215/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1215(t113111: f64, t113117: f64, t118399: f64, t118948: f64, t1484: f64, t1530: f64, t16596: f64, t1877: f64, t193: f64, t202: f64, t2522: f64, t25365: f64, t25374: f64, t30753: f64, t30757: f64, t32886: f64, t4119: f64, t4255: f64, t4303: f64, t4314: f64, t776: f64, t8366: f64, t868: f64, t870: f64) -> f64 {
    let t119639 = t118948 * t193 * t202 * t870 - t113111 * t1530 * t1877 + 2.0_f64 * t113117 * t1877 * t25374 - t118399 * t1877 * t868 + 3.0_f64 * t1484 * t2522 * t30753 - 3.0_f64 * t16596 * t2522 * t30757 - t1877 * t30757 * t4303 - 3.0_f64 * t2522 * t25365 * t30757 + 3.0_f64 * t2522 * t32886 * t776 + 3.0_f64 * t2522 * t4119 * t8366 + 6.0_f64 * t4255 * t4314 * t8366;
    t119639
}
