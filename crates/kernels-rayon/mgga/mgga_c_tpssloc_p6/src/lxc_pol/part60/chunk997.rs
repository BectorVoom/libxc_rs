//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 997/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk997(t10110: f64, t114760: f64, t121371: f64, t126249: f64, t126264: f64, t126278: f64, t127798: f64, t127803: f64, t127814: f64, t127818: f64, t127829: f64, t1527: f64, t25348: f64, t26713: f64, t2718: f64, t29060: f64, t33398: f64, t33433: f64, t4147: f64, t4268: f64, t5636: f64, t6627: f64, t7517: f64, t7830: f64, t855: f64, t8562: f64) -> f64 {
    let t127833 = -0.82246703342411321825e-2_f64 * t127798 - t126249 + 4.0_f64 * t25348 * t7830 - 0.82246703342411321825e-2_f64 * t127803 + 4.0_f64 * t855 * t2718 * t33398 * t1527 + t126264 - 0.76763589786250567036e-1_f64 * t121371 + 4.0_f64 * t4147 * t33433 + 0.16449340668482264365e-1_f64 * t127814 + t114760 - 0.6579736267392905746e-1_f64 * t127818 - t126278 + 4.0_f64 * t4268 * t33433 + 2.0_f64 * t6627 * t29060 - 6.0_f64 * t855 * t10110 * t8562 * t5636 + 0.3289868133696452873e-1_f64 * t127829 + 4.0_f64 * t26713 * t7517;
    t127833
}
