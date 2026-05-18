//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 794/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk794<F: Float>(t1422: F, t1434: F, t2538: F, t2560: F, t2577: F, t2599: F, t3527: F, t3560: F, t374: F, t4240: F, t4242: F, t4246: F, t4272: F, t4275: F, t4278: F, t4284: F, t4297: F, t4300: F, t4306: F, t4311: F, t4324: F, t4327: F, t979: F, t998: F) -> F {
    let t4330 = -F::new(0.310907e-1) * t4278 * t374 + F::new(2.0) * t3527 * t1422 - F::new(2.0) * t2538 * t4284 + F::new(1.0) * t979 * t4297 + F::new(0.32163958997385070134e2) * t2560 * t4300 + t4240 - t4242 + t4246 - t4272 - t4275 - F::new(0.19751673498613801407e-1) * t4306 + F::new(0.11696447245269292414e1) * t3560 * t1434 - F::new(0.11696447245269292414e1) * t2577 * t4311 + F::new(0.5848223622634646207e0) * t998 * t4324 + F::new(0.17315859105681463759e2) * t2599 * t4327;
    t4330
}
