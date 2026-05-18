//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 376/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk376<F: Float>(t1335: F, t1349: F, t1351: F, t1359: F, t1364: F, t1371: F, t1379: F, t260: F, t271: F, t821: F, t840: F, t856: F) -> F {
    let t1382 = -t1335 + t1349 + t260 * (-F::new(0.310907e-1) * t1351 * t271 + F::new(1.0) * t821 * t1359 + t1335 - t1349 - F::new(0.19751673498613801407e-1) * t1364 + F::new(0.5848223622634646207e0) * t840 * t1371) + F::new(0.19751673498613801407e-1) * t260 * t1364 - F::new(0.5848223622634646207e0) * t856 * t1379;
    t1382
}
