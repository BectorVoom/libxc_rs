//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1263/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1263<F: Float>(t30860: F, t30884: F, t30916: F, t30940: F, t787: F, t809: F, t11113: F, t1365: F, t22498: F, t22501: F, t22580: F, t22733: F, t2284: F, t2286: F, t2301: F, t2303: F, t2318: F, t2326: F, t26298: F, t26301: F, t26304: F, t26607: F, t26616: F, t26634: F, t26752: F, t271: F, t30747: F, t30750: F, t30778: F, t30792: F, t30795: F, t30797: F, t30801: F, t30805: F, t30809: F, t30816: F, t30821: F, t30831: F, t3416: F, t3436: F, t4246: F, t829: F, t846: F, t847: F, t9053: F, t9071: F, t9166: F) -> (F, F) {
    let t30945 = 1.0 * t787 * (t30860 + t30884 + t30916 + t30940) * t809;
    let t30946 = -0.310907e-1 * (t22733 - 0.10654518518518518518e0 * t22498 + 0.22831111111111111111e-1 * t22501 - 0.10654518518518518518e0 * t26298 + 0.91324444444444444442e-1 * t26301 - 0.34246666666666666666e-1 * t26304 + 0.22831111111111111111e-1 * t30747 - 0.34246666666666666666e-1 * t30750 + 0.5137e-1 * t30778) * t271 - 0.19751673498613801407e-1 * t30792 - t30795 + t30797 + 0.41016075432865626631e4 * t26616 * t26752 * t846 - 0.23392894490538584828e1 * t2301 * t30801 * t847 + 0.64327917994770140268e2 * t2284 * t30805 * t2286 - 0.11696447245269292414e1 * t30809 * t2303 + 24.0 * t26607 * t9166 + 0.17315859105681463759e2 * t22580 * t4246 + 2.0 * t30816 * t829 + 0.5848223622634646207e0 * t11113 * t2318 + 0.17315859105681463759e2 * t30821 * t2326 + 0.11696447245269292414e1 * t26634 * t1365 + 0.23392894490538584828e1 * t9071 * t3436 + 0.11696447245269292414e1 * t3416 * t9053 - t30831 - t30945;
    (t30945, t30946)
}
