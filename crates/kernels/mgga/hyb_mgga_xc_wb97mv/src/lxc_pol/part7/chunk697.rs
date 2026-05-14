//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 697/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk697<F: Float>(t3600: F, t994: F, t1416: F, t1428: F, t2529: F, t2534: F, t2556: F, t2568: F, t2573: F, t2595: F, t3493: F, t3496: F, t3498: F, t3501: F, t3533: F, t3537: F, t3541: F, t3544: F, t3549: F, t3564: F, t3568: F, t3575: F, t3577: F, t3582: F, t3597: F, t372: F, t968: F, t977: F, t987: F, t996: F) -> (F, F) {
    let t3601 = t3600 * t994;
    let t3604 = -0.310907e-1 * t3541 * t372 + 1.0 * t3544 * t977 + 1.0 * t2529 * t1416 - 2.0 * t2534 * t3549 + 1.0 * t968 * t3564 + 0.32163958997385070134e2 * t2556 * t3568 + t3493 - t3496 - t3498 + t3501 - t3533 - t3537 - 0.19751673498613801407e-1 * t3575 + 0.5848223622634646207e0 * t3577 * t996 + 0.5848223622634646207e0 * t2568 * t1428 - 0.11696447245269292414e1 * t2573 * t3582 + 0.5848223622634646207e0 * t987 * t3597 + 0.17315859105681463759e2 * t2595 * t3601;
    (t3601, t3604)
}
