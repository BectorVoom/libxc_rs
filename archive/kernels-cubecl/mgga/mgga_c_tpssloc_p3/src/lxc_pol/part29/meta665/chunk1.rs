//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2211/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2211<F: Float>(t1992: F, t54854: F, t550: F, t6976: F, t26331: F, t26421: F, t26446: F, t3719: F, t22704: F, t22705: F, t26466: F, t81022: F, t90806: F, t90807: F, t90812: F, t90816: F, t90821: F, t90825: F, t90829: F, t90832: F, t90835: F, t90837: F, t90840: F, t90845: F, t90848: F) -> F {
    let t90852 = t1992 * t6976 * t54854 * t550;
    let t90856 = t26331 * t26446 * t26421 * t3719;
    let t90859 = t22704 * t22705 * t26466;
    let t90860 = F::cast_from(0.82246703342411321824e-2_f64) * t90859;
    let t90861 = t90806 - F::cast_from(0.12793931631041761173e0_f64) * t90807 - F::cast_from(0.3289868133696452873e-1_f64) * t90812 + F::cast_from(0.3289868133696452873e-1_f64) * t90816 + F::cast_from(0.3289868133696452873e-1_f64) * t90821 - F::cast_from(0.16449340668482264365e-1_f64) * t90825 - F::cast_from(0.3289868133696452873e-1_f64) * t90829 - F::cast_from(0.49348022005446793095e-1_f64) * t90832 + F::cast_from(0.49348022005446793095e-1_f64) * t90835 - F::cast_from(0.52089578783527170489e-1_f64) * t90837 - F::cast_from(0.16449340668482264365e-1_f64) * t90840 - F::cast_from(0.82246703342411321824e-2_f64) * t81022 - t90845 + F::cast_from(0.3289868133696452873e-1_f64) * t90848 - F::cast_from(0.82246703342411321825e-2_f64) * t90852 + F::cast_from(0.49348022005446793095e-1_f64) * t90856 + t90860;
    t90861
}
