//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1311/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1311(t23185: f64, t32862: f64, t82074: f64, t32863: f64, t6579: f64, t112726: f64, t112660: f64, t6552: f64, t7479: f64, t112961: f64, t32823: f64, t1888: f64, t22996: f64, t25281: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t118661 = t23185 * t82074 * t32862;
    let t118662 = 0.16449340668482264365e-1_f64 * t118661;
    let t118663 = t6579 * t32863;
    let t118664 = 0.76763589786250567037e-1_f64 * t118663;
    let t118667 = 0.38381794893125283518e-1_f64 * t112726;
    let t118672 = 0.3289868133696452873e-1_f64 * t6552 * t112660 * t7479;
    let t118677 = 0.16449340668482264365e-1_f64 * t112961;
    let t118678 = t6579 * t32823;
    let t118679 = 0.38381794893125283518e-1_f64 * t118678;
    let t118682 = 0.3289868133696452873e-1_f64 * t1888 * t22996 * t25281;
    (t118662, t118664, t118667, t118672, t118677, t118679, t118682)
}
