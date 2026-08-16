//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2013/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2013(t12021: f64, t1375: f64, t1843: f64, t20060: f64, t24082: f64, t29311: f64, t29372: f64, t3882: f64, t6439: f64, t6440: f64, t7199: f64, t7213: f64, t81264: f64, t90642: f64, t93338: f64, t93439: f64, t97513: f64, t97516: f64) -> f64 {
    let t102523 = -2.0_f64 * t93338 * t1843 - 0.16449340668482264365e-1_f64 * t97513 + 0.6579736267392905746e-1_f64 * t97516 + 0.3289868133696452873e-1_f64 * t90642 + 2.0_f64 * t24082 * t6440 + t93439 - 6.0_f64 * t1375 * t12021 * t7213 * t6439 + 4.0_f64 * t3882 * t29311 + 2.0_f64 * t20060 * t7199 + 0.52089578783527170489e-1_f64 * t81264 + 2.0_f64 * t3882 * t29372;
    t102523
}
