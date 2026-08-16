//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1135/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1135(t12012: f64, t12156: f64, t12477: f64, t1307: f64, t1388: f64, t1390: f64, t193: f64, t3719: f64, t3918: f64, t39529: f64, t39531: f64, t39533: f64, t39539: f64, t39541: f64, t39549: f64, t39563: f64, t39570: f64, t39572: f64, t39577: f64, t39582: f64, t39585: f64, t5126: f64, t571: f64) -> f64 {
    let t39586 = 24.0_f64 * t12012 * t1307 * t5126 * t571 + 24.0_f64 * t12156 * t1388 * t1390 * t193 - 18.0_f64 * t12477 * t3719 * t3918 + 12.0_f64 * t1307 * t3918 * t39577 - t39529 - t39531 - t39533 + t39539 - t39541 + t39549 + t39563 + t39570 - t39572 + t39582 - t39585;
    t39586
}
