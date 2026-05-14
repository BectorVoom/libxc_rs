//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 679/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk679<F: Float>(t3460: F, t3461: F, t1373: F, t2333: F, t260: F, t3332: F, t3335: F, t3337: F, t3340: F, t3372: F, t3376: F, t3414: F, t3443: F, t3447: F, t3453: F, t3457: F, t855: F, t857: F) -> (F, F) {
    let t3462 = t3460 * t3461;
    let t3465 = -t3332 + t3335 + t3337 - t3340 + t3372 + t3376 + t260 * t3443 + 0.19751673498613801407e-1 * t260 * t3414 - 0.5848223622634646207e0 * t3447 * t857 - 0.5848223622634646207e0 * t2333 * t1373 + 0.11696447245269292414e1 * t855 * t3453 - 0.5848223622634646207e0 * t855 * t3457 - 0.17315859105681463759e2 * t855 * t3462;
    (t3462, t3465)
}
