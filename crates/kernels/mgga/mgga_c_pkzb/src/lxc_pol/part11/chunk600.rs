//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 600/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk600<F: Float>(t24: F, t3004: F, t1263: F, t1265: F, t3289: F, t422: F, t423: F, t960: F, t962: F, t330: F, t987: F, t995: F, t973: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t90 = t24 <= zeta_threshold;
    let t332 = rho1 <= dens_threshold || t90;
    let t3293 = piecewise3::<F>(t90, F::cast_from(0.0_f64), -t3004);
    let t3297 = piecewise3::<F>(t332, F::cast_from(0.0_f64), t1263 * t962 / F::cast_from(2.0_f64) + t960 * t1265 / F::cast_from(2.0_f64) + t3289 * t423 / F::cast_from(2.0_f64) + t422 * t3293 / F::cast_from(2.0_f64));
    let t3298 = t330 * t3297;
    let t3308 = t987 * t987;
    let t3311 = t987 * t995;
    let t3314 = t973 * t973;
    (t3293, t3298, t3308, t3311, t3314)
}
