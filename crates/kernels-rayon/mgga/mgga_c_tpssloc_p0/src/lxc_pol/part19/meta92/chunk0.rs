//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 523/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk523(t218: f64, t2710: f64, t225: f64, t853: f64, t257: f64, t856: f64, t68: f64, t865: f64, t252: f64, t2627: f64, t2633: f64, t814: f64, t852: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2711 = t218 * t2710;
    let t2713 = t853 * t225;
    let t2717 = 1.0_f64 / t856 / t257;
    let t2718 = t68 * t2717;
    let t2719 = t865 * t865;
    let t2720 = t2718 * t2719;
    let t2728 = t2627 * t252;
    let t2729 = t2728 * t2633;
    let t2732 = t814 * t852;
    (t2711, t2713, t2718, t2719, t2720, t2728, t2729, t2732)
}
