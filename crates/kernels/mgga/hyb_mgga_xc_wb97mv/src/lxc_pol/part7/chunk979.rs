//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 979/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk979<F: Float>(t3535: F, t7371: F, t3531: F, t956: F, t2474: F, t1404: F, t2511: F, t2475: F, t3534: F, t7376: F, t2519: F, t3530: F, t2517: F, t2573: F, t2595: F, t7259: F, t9558: F, t9561: F, t9565: F, t9568: F, t9573: F, t9577: F, t9579: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9581 = 0.32163958997385070134e2 * t7371 * t3535;
    let t9582 = t3531 * t956;
    let t9584 = 4.0 * t2474 * t9582;
    let t9585 = t1404 * t2511;
    let t9587 = 2.0 * t2474 * t9585;
    let t9588 = t3534 * t2475;
    let t9590 = 0.96491876992155210402e2 * t7376 * t9588;
    let t9591 = t3530 * t2519;
    let t9592 = t9591 * t956;
    let t9594 = 0.32163958997385070134e2 * t2517 * t9592;
    let t9595 = t1404 * t2475;
    let t9597 = 6.0 * t2517 * t9595;
    let t9598 = -0.11696447245269292414e1 * t2573 * t9558 - 0.10389515463408878255e3 * t7259 * t9561 + 0.34631718211362927518e2 * t2595 * t9565 + 0.17315859105681463759e2 * t2595 * t9568 - t9573 - t9577 + t9579 - t9581 + t9584 + t9587 + t9590 - t9594 - t9597;
    (t9581, t9582, t9584, t9585, t9587, t9588, t9590, t9592, t9594, t9595, t9597, t9598)
}
