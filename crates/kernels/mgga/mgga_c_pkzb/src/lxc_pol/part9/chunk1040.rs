//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1040/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1040<F: Float>(t24: F, t8567: F, t8576: F, t7908: F, t1263: F, t1265: F, t2467: F, t2471: F, t3289: F, t3293: F, t422: F, t423: F, t960: F, t962: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F) {
    let t90 = t24 <= zeta_threshold;
    let t332 = rho1 <= dens_threshold || t90;
    let t8577 = t8567 + t8576;
    let t8587 = piecewise3::<F>(t90, F::cast_from(0.0_f64), -t7908);
    let t8591 = piecewise3::<F>(t332, F::cast_from(0.0_f64), t8577 * t423 / F::cast_from(2.0_f64) + t3289 * t962 + t1263 * t2471 / F::cast_from(2.0_f64) + t2467 * t1265 / F::cast_from(2.0_f64) + t960 * t3293 + t422 * t8587 / F::cast_from(2.0_f64));
    (t8577, t8587, t8591)
}
