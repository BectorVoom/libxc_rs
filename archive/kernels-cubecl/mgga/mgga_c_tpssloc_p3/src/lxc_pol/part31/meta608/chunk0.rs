//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1853/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1853<F: Float>(t90791: F, t90794: F, t90797: F, t90805: F, t90844: F, t90859: F, t90864: F, t90866: F, t90898: F, t90912: F, t90956: F, t90961: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t93489 = F::cast_from(0.15352717957250113407e0_f64) * t90791;
    let t93490 = F::cast_from(0.3289868133696452873e-1_f64) * t90794;
    let t93491 = F::cast_from(0.3289868133696452873e-1_f64) * t90797;
    let t93494 = F::cast_from(0.3289868133696452873e-1_f64) * t90805;
    let t93524 = F::cast_from(0.3289868133696452873e-1_f64) * t90844;
    let t93528 = F::cast_from(0.16449340668482264365e-1_f64) * t90859;
    let t93529 = F::cast_from(0.16449340668482264365e-1_f64) * t90864;
    let t93537 = F::cast_from(0.76763589786250567036e-1_f64) * t90866;
    let t93562 = F::cast_from(0.3289868133696452873e-1_f64) * t90898;
    let t93572 = F::cast_from(0.15352717957250113407e0_f64) * t90912;
    let t93588 = F::cast_from(0.76763589786250567036e-1_f64) * t90956;
    let t93589 = F::cast_from(0.3289868133696452873e-1_f64) * t90961;
    (t93489, t93490, t93491, t93494, t93524, t93528, t93529, t93537, t93562, t93572, t93588, t93589)
}
