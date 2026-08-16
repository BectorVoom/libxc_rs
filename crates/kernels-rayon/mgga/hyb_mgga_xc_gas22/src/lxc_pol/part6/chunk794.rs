//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 794/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk794(t1422: f64, t1434: f64, t2538: f64, t2560: f64, t2577: f64, t2599: f64, t3527: f64, t3560: f64, t374: f64, t4240: f64, t4242: f64, t4246: f64, t4272: f64, t4275: f64, t4278: f64, t4284: f64, t4297: f64, t4300: f64, t4306: f64, t4311: f64, t4324: f64, t4327: f64, t979: f64, t998: f64) -> f64 {
    let t4330 = -0.310907e-1_f64 * t4278 * t374 + 2.0_f64 * t3527 * t1422 - 2.0_f64 * t2538 * t4284 + 1.0_f64 * t979 * t4297 + 0.32163958997385070134e2_f64 * t2560 * t4300 + t4240 - t4242 + t4246 - t4272 - t4275 - 0.19751673498613801407e-1_f64 * t4306 + 0.11696447245269292414e1_f64 * t3560 * t1434 - 0.11696447245269292414e1_f64 * t2577 * t4311 + 0.5848223622634646207e0_f64 * t998 * t4324 + 0.17315859105681463759e2_f64 * t2599 * t4327;
    t4330
}
