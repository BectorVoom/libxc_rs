//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1848/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1848<F: Float>(t87583: F, t87601: F, t87603: F, t87612: F, t87618: F, t87668: F, t87679: F, t87709: F, t87714: F, t87729: F, t87733: F, t87753: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t92739 = F::cast_from(0.15352717957250113407e0_f64) * t87583;
    let t92749 = F::cast_from(0.16449340668482264365e-1_f64) * t87601;
    let t92754 = F::cast_from(0.15352717957250113407e0_f64) * t87603;
    let t92760 = F::cast_from(0.3289868133696452873e-1_f64) * t87612;
    let t92768 = F::cast_from(0.3289868133696452873e-1_f64) * t87618;
    let t92795 = F::cast_from(0.76763589786250567036e-1_f64) * t87668;
    let t92798 = F::cast_from(0.3289868133696452873e-1_f64) * t87679;
    let t92810 = F::cast_from(0.76763589786250567036e-1_f64) * t87709;
    let t92811 = F::cast_from(0.9869604401089358619e-1_f64) * t87714;
    let t92822 = F::cast_from(0.16449340668482264365e-1_f64) * t87729;
    let t92825 = F::cast_from(0.76763589786250567036e-1_f64) * t87733;
    let t92846 = F::cast_from(0.3289868133696452873e-1_f64) * t87753;
    (t92739, t92749, t92754, t92760, t92768, t92795, t92798, t92810, t92811, t92822, t92825, t92846)
}
