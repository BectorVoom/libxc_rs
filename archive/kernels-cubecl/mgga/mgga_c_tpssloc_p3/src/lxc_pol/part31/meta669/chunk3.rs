//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1980/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1980<F: Float>(t87155: F, t92515: F, t92530: F, t98439: F, t98443: F, t98446: F, t98461: F, t98464: F, t98467: F, t98471: F, t98475: F, t98478: F, t98482: F, t98486: F, t98488: F, t98490: F, t98502: F) -> F {
    let t101672 = F::cast_from(0.6579736267392905746e-1_f64) * t98439 + F::cast_from(0.6579736267392905746e-1_f64) * t98443 - F::cast_from(0.3289868133696452873e-1_f64) * t98446 + F::cast_from(0.6579736267392905746e-1_f64) * t98461 + F::cast_from(0.6579736267392905746e-1_f64) * t98464 + F::cast_from(0.3289868133696452873e-1_f64) * t98467 - t92515 + F::cast_from(0.10417915756705434098e0_f64) * t87155 + F::cast_from(0.6579736267392905746e-1_f64) * t98471 - F::cast_from(0.6579736267392905746e-1_f64) * t98475 + F::cast_from(0.6579736267392905746e-1_f64) * t98478 - F::cast_from(0.3289868133696452873e-1_f64) * t98482 + F::cast_from(0.3289868133696452873e-1_f64) * t98486 + F::cast_from(0.38381794893125283518e-1_f64) * t98488 - F::cast_from(0.76763589786250567037e-1_f64) * t98490 + F::cast_from(0.19739208802178717238e0_f64) * t98502 + t92530;
    t101672
}
