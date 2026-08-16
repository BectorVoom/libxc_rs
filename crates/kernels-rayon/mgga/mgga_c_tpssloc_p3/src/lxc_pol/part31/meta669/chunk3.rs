//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1980/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1980(t87155: f64, t92515: f64, t92530: f64, t98439: f64, t98443: f64, t98446: f64, t98461: f64, t98464: f64, t98467: f64, t98471: f64, t98475: f64, t98478: f64, t98482: f64, t98486: f64, t98488: f64, t98490: f64, t98502: f64) -> f64 {
    let t101672 = 0.6579736267392905746e-1_f64 * t98439 + 0.6579736267392905746e-1_f64 * t98443 - 0.3289868133696452873e-1_f64 * t98446 + 0.6579736267392905746e-1_f64 * t98461 + 0.6579736267392905746e-1_f64 * t98464 + 0.3289868133696452873e-1_f64 * t98467 - t92515 + 0.10417915756705434098e0_f64 * t87155 + 0.6579736267392905746e-1_f64 * t98471 - 0.6579736267392905746e-1_f64 * t98475 + 0.6579736267392905746e-1_f64 * t98478 - 0.3289868133696452873e-1_f64 * t98482 + 0.3289868133696452873e-1_f64 * t98486 + 0.38381794893125283518e-1_f64 * t98488 - 0.76763589786250567037e-1_f64 * t98490 + 0.19739208802178717238e0_f64 * t98502 + t92530;
    t101672
}
