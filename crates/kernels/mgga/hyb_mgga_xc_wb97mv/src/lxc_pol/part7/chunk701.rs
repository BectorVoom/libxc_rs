//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 701/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk701<F: Float>(t3621: F, t3622: F, t1003: F, t1005: F, t1436: F, t260: F, t2605: F, t3493: F, t3496: F, t3498: F, t3501: F, t3533: F, t3537: F, t3575: F, t3604: F, t3608: F, t3614: F, t3618: F) -> (F, F) {
    let t3623 = t3621 * t3622;
    let t3626 = -t3493 + t3496 + t3498 - t3501 + t3533 + t3537 + t260 * t3604 + 0.19751673498613801407e-1 * t260 * t3575 - 0.5848223622634646207e0 * t3608 * t1005 - 0.5848223622634646207e0 * t2605 * t1436 + 0.11696447245269292414e1 * t1003 * t3614 - 0.5848223622634646207e0 * t1003 * t3618 - 0.17315859105681463759e2 * t1003 * t3623;
    (t3623, t3626)
}
