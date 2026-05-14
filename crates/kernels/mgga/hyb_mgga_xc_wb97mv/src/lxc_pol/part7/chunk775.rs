//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 775/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk775<F: Float>(t1353: F, t1365: F, t2262: F, t2284: F, t2301: F, t2323: F, t271: F, t3383: F, t3416: F, t4159: F, t4161: F, t4165: F, t4191: F, t4194: F, t4197: F, t4203: F, t4216: F, t4219: F, t4225: F, t4230: F, t4243: F, t4246: F, t820: F, t839: F) -> (F,) {
    let t4249 = -0.310907e-1 * t4197 * t271 + 2.0 * t3383 * t1353 - 2.0 * t2262 * t4203 + 1.0 * t820 * t4216 + 0.32163958997385070134e2 * t2284 * t4219 + t4159 - t4161 + t4165 - t4191 - t4194 - 0.19751673498613801407e-1 * t4225 + 0.11696447245269292414e1 * t3416 * t1365 - 0.11696447245269292414e1 * t2301 * t4230 + 0.5848223622634646207e0 * t839 * t4243 + 0.17315859105681463759e2 * t2323 * t4246;
    (t4249,)
}
