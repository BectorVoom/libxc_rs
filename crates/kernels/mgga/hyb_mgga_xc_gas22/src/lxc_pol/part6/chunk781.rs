//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 781/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk781<F: Float>(t1006: F, t2576: F, t4310: F, t4323: F, t997: F, t2598: F, t2601: F, t1014: F, t1442: F, t260: F, t3591: F, t4240: F, t4242: F, t4246: F, t4272: F, t4275: F, t4306: F, t4330: F) -> (F, F, F, F, F) {
    let t4337 = t2576 * t4310 * t1006;
    let t4341 = t997 * t4323 * t1006;
    let t4344 = t2598 * t4310;
    let t4345 = t4344 * t2601;
    let t4348 = -t4240 + t4242 - t4246 + t4272 + t4275 + t260 * t4330 + 0.19751673498613801407e-1 * t260 * t4306 - 0.11696447245269292414e1 * t3591 * t1442 + 0.11696447245269292414e1 * t1014 * t4337 - 0.5848223622634646207e0 * t1014 * t4341 - 0.17315859105681463759e2 * t1014 * t4345;
    (t4337, t4341, t4344, t4345, t4348)
}
