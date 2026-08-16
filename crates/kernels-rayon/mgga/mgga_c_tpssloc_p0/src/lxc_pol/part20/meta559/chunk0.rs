//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2115/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2115(t42101: f64, t10619: f64, t942: f64, t2928: f64, t315: f64, t2931: f64, t10843: f64, t923: f64, t2853: f64, t2885: f64, t10523: f64, t938: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t42102 = 1.0_f64 / t42101;
    let t42106 = t10619 * t942;
    let t42109 = t2928 * t2928;
    let t42110 = 1.0_f64 / t42109;
    let t42111 = t315 * t42110;
    let t42112 = t2931 * t2931;
    let t42113 = 1.0_f64 / t42112;
    let t42117 = t10843 * t923;
    let t42123 = t2853 * t2885;
    let t42128 = t938 * t10523;
    (t42102, t42106, t42110, t42111, t42113, t42117, t42123, t42128)
}
