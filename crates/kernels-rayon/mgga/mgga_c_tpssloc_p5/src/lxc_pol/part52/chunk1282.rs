//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1282/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1282(t112660: f64, t6552: f64, t7479: f64, t112961: f64, t32823: f64, t6579: f64, t1888: f64, t22996: f64, t25281: f64, t1509: f64, t8347: f64, t1484: f64, t1902: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t118672 = 0.3289868133696452873e-1_f64 * t6552 * t112660 * t7479;
    let t118677 = 0.16449340668482264365e-1_f64 * t112961;
    let t118678 = t6579 * t32823;
    let t118679 = 0.38381794893125283518e-1_f64 * t118678;
    let t118682 = 0.3289868133696452873e-1_f64 * t1888 * t22996 * t25281;
    let t118684 = t8347 * t1509;
    let t118690 = t1902 * t1484;
    (t118672, t118677, t118679, t118682, t118684, t118690)
}
