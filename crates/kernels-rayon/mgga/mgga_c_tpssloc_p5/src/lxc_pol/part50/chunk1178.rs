//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1178/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1178(t118661: f64, t32863: f64, t6579: f64, t112726: f64, t118639: f64, t118640: f64, t118650: f64, t118654: f64, t13042: f64, t13463: f64, t1912: f64, t23281: f64, t25170: f64, t2713: f64, t32800: f64, t32804: f64, t7517: f64, t8363: f64, t87758: f64, t87810: f64, t87837: f64) -> f64 {
    let t118662 = 0.16449340668482264365e-1_f64 * t118661;
    let t118663 = t6579 * t32863;
    let t118664 = 0.76763589786250567037e-1_f64 * t118663;
    let t118667 = 0.38381794893125283518e-1_f64 * t112726;
    let t118668 = -12.0_f64 * t118640 * t25170 - t13042 * t8363 - t13463 * t8363 - 2.0_f64 * t1912 * t87758 - 2.0_f64 * t1912 * t87810 - 2.0_f64 * t1912 * t87837 + 4.0_f64 * t23281 * t7517 + 4.0_f64 * t2713 * t32800 + 2.0_f64 * t2713 * t32804 + t118639 + t118650 + t118654 - t118662 - t118664 + t118667;
    t118668
}
