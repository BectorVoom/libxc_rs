//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1978/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1978<F: Float>(t87073: F, t87078: F, t87080: F, t92502: F, t98356: F, t98359: F, t98363: F, t98367: F, t98374: F, t98380: F, t98384: F, t98387: F, t98392: F, t98396: F, t98399: F, t98402: F, t98405: F) -> F {
    let t101634 = F::cast_from(0.16449340668482264365e-1_f64) * t98356 - F::cast_from(0.13159472534785811492e0_f64) * t98359 + t87073 - F::cast_from(0.16449340668482264365e-1_f64) * t98363 - F::cast_from(0.39478417604357434476e0_f64) * t98367 - F::cast_from(0.46058153871750340221e0_f64) * t87078 - F::cast_from(0.38381794893125283518e-1_f64) * t98374 + F::cast_from(0.25587863262083522345e0_f64) * t87080 + F::cast_from(0.38381794893125283518e-1_f64) * t98380 - F::cast_from(0.3289868133696452873e-1_f64) * t98384 - F::cast_from(0.16449340668482264365e-1_f64) * t98387 + F::cast_from(0.19739208802178717238e0_f64) * t98392 - F::cast_from(0.16449340668482264365e-1_f64) * t98396 + F::cast_from(0.82246703342411321825e-2_f64) * t98399 - F::cast_from(0.9869604401089358619e-1_f64) * t98402 + F::cast_from(0.9869604401089358619e-1_f64) * t98405 + t92502;
    t101634
}
