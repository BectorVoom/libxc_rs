//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 793/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk793<F: Float>(t1416: F, t1428: F, t2534: F, t2556: F, t2573: F, t2595: F, t3544: F, t3577: F, t372: F, t4289: F, t4291: F, t4295: F, t4321: F, t4324: F, t4327: F, t4333: F, t4346: F, t4349: F, t4355: F, t4360: F, t4373: F, t4376: F, t968: F, t987: F) -> (F,) {
    let t4379 = -0.310907e-1 * t4327 * t372 + 2.0 * t3544 * t1416 - 2.0 * t2534 * t4333 + 1.0 * t968 * t4346 + 0.32163958997385070134e2 * t2556 * t4349 + t4289 - t4291 + t4295 - t4321 - t4324 - 0.19751673498613801407e-1 * t4355 + 0.11696447245269292414e1 * t3577 * t1428 - 0.11696447245269292414e1 * t2573 * t4360 + 0.5848223622634646207e0 * t987 * t4373 + 0.17315859105681463759e2 * t2595 * t4376;
    (t4379,)
}
