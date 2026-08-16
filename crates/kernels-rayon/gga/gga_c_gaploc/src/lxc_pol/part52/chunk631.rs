//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 631/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk631(t11576: f64, t836: f64, t568: f64, t10010: f64, t11834: f64, t11837: f64, t11841: f64, t11845: f64, t11849: f64, t11854: f64, t2028: f64, t2103: f64, t2197: f64, t3651: f64, t3677: f64, t3681: f64, t5748: f64, t5775: f64, t5782: f64, t6148: f64, t807: f64, t833: f64) -> f64 {
    let t11861 = t836 * t11576;
    let t11862 = t568 * t11861;
    let t11866 = 0.27606906686822939767e2_f64 * t5748 * t11834 + 0.23005755572352449806e1_f64 * t807 * t11837 + 0.69017266717057349418e1_f64 * t6148 * t11841 - 0.39722766613167140743e-1_f64 * t11845 * t2028 - 0.39722766613167140743e-1_f64 * t11849 * t2028 - 0.7150097990370085334e0_f64 * t3651 * t5775 + 0.47667319935800568892e0_f64 * t2103 * t11854 - 0.69017266717057349418e1_f64 * t5782 * t3677 + 0.23005755572352449806e1_f64 * t2197 * t3681 + 0.23005755572352449806e1_f64 * t833 * t11862 - 0.63904876589867916126e-1_f64 * t10010;
    t11866
}
