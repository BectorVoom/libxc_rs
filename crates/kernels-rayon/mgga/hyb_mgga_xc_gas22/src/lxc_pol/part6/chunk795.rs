//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 795/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk795(t1006: f64, t2576: f64, t4310: f64, t4323: f64, t997: f64, t2598: f64, t2601: f64, t1014: f64, t1442: f64, t260: f64, t3591: f64, t4240: f64, t4242: f64, t4246: f64, t4272: f64, t4275: f64, t4306: f64, t4330: f64) -> (f64, f64, f64, f64, f64) {
    let t4337 = t2576 * t4310 * t1006;
    let t4341 = t997 * t4323 * t1006;
    let t4344 = t2598 * t4310;
    let t4345 = t4344 * t2601;
    let t4348 = -t4240 + t4242 - t4246 + t4272 + t4275 + t260 * t4330 + 0.19751673498613801407e-1_f64 * t260 * t4306 - 0.11696447245269292414e1_f64 * t3591 * t1442 + 0.11696447245269292414e1_f64 * t1014 * t4337 - 0.5848223622634646207e0_f64 * t1014 * t4341 - 0.17315859105681463759e2_f64 * t1014 * t4345;
    (t4337, t4341, t4344, t4345, t4348)
}
