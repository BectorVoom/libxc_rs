//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1052/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1052(t1375: f64, t22664: f64, t22668: f64, t22676: f64, t22688: f64, t22907: f64, t22909: f64, t22918: f64, t22921: f64, t22928: f64, t22931: f64, t22936: f64, t22940: f64, t24139: f64, t24141: f64, t24147: f64, t24156: f64, t24157: f64, t24162: f64, t568: f64) -> f64 {
    let t24164 = -t1375 * t24139 + 2.0_f64 * t24141 * t568 - 0.16449340668482264365e-1_f64 * t22664 - 0.3289868133696452873e-1_f64 * t22668 + 4.0_f64 * t1375 * t24147 + 0.16449340668482264365e-1_f64 * t22676 + 0.9869604401089358619e-1_f64 * t22688 + 0.15352717957250113407e0_f64 * t22907 + 0.76763589786250567036e-1_f64 * t22909 - 0.3289868133696452873e-1_f64 * t22918 + 0.3289868133696452873e-1_f64 * t22921 + t24156 + t24157 - 0.16449340668482264365e-1_f64 * t22928 - 0.6579736267392905746e-1_f64 * t22931 + 0.3289868133696452873e-1_f64 * t22936 - 0.76763589786250567036e-1_f64 * t22940 + t24162 * t568;
    t24164
}
