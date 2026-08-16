//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1052/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1052<F: Float>(t1375: F, t22664: F, t22668: F, t22676: F, t22688: F, t22907: F, t22909: F, t22918: F, t22921: F, t22928: F, t22931: F, t22936: F, t22940: F, t24139: F, t24141: F, t24147: F, t24156: F, t24157: F, t24162: F, t568: F) -> F {
    let t24164 = -t1375 * t24139 + F::cast_from(2.0_f64) * t24141 * t568 - F::cast_from(0.16449340668482264365e-1_f64) * t22664 - F::cast_from(0.3289868133696452873e-1_f64) * t22668 + F::cast_from(4.0_f64) * t1375 * t24147 + F::cast_from(0.16449340668482264365e-1_f64) * t22676 + F::cast_from(0.9869604401089358619e-1_f64) * t22688 + F::cast_from(0.15352717957250113407e0_f64) * t22907 + F::cast_from(0.76763589786250567036e-1_f64) * t22909 - F::cast_from(0.3289868133696452873e-1_f64) * t22918 + F::cast_from(0.3289868133696452873e-1_f64) * t22921 + t24156 + t24157 - F::cast_from(0.16449340668482264365e-1_f64) * t22928 - F::cast_from(0.6579736267392905746e-1_f64) * t22931 + F::cast_from(0.3289868133696452873e-1_f64) * t22936 - F::cast_from(0.76763589786250567036e-1_f64) * t22940 + t24162 * t568;
    t24164
}
