//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1082/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1082<F: Float>(t11374: F, t986: F, t995: F, t4354: F, t11253: F, t11255: F, t11325: F, t11327: F, t11338: F, t11340: F, t11342: F, t1428: F, t2568: F, t3577: F, t3597: F, t4360: F, t4373: F, t7409: F, t9452: F, t996: F) -> (F, F, F) {
    let t11376 = t986 * t11374 * t995;
    let t11383 = t4354 * t986;
    let t11394 = t11253 - t11255 - t11325 - t11327 - t11338 - t11340 - t11342 + 0.5848223622634646207e0 * t11383 * t996 + 0.11696447245269292414e1 * t9452 * t1428 + 0.11696447245269292414e1 * t3577 * t3597 - 0.11696447245269292414e1 * t7409 * t4360 + 0.5848223622634646207e0 * t2568 * t4373;
    (t11376, t11383, t11394)
}
