//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1978/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1978(t87073: f64, t87078: f64, t87080: f64, t92502: f64, t98356: f64, t98359: f64, t98363: f64, t98367: f64, t98374: f64, t98380: f64, t98384: f64, t98387: f64, t98392: f64, t98396: f64, t98399: f64, t98402: f64, t98405: f64) -> f64 {
    let t101634 = 0.16449340668482264365e-1_f64 * t98356 - 0.13159472534785811492e0_f64 * t98359 + t87073 - 0.16449340668482264365e-1_f64 * t98363 - 0.39478417604357434476e0_f64 * t98367 - 0.46058153871750340221e0_f64 * t87078 - 0.38381794893125283518e-1_f64 * t98374 + 0.25587863262083522345e0_f64 * t87080 + 0.38381794893125283518e-1_f64 * t98380 - 0.3289868133696452873e-1_f64 * t98384 - 0.16449340668482264365e-1_f64 * t98387 + 0.19739208802178717238e0_f64 * t98392 - 0.16449340668482264365e-1_f64 * t98396 + 0.82246703342411321825e-2_f64 * t98399 - 0.9869604401089358619e-1_f64 * t98402 + 0.9869604401089358619e-1_f64 * t98405 + t92502;
    t101634
}
