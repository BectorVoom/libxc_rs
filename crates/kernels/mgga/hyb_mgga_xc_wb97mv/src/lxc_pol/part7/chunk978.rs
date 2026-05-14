//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 978/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk978<F: Float>(t1428: F, t2589: F, t2574: F, t3600: F, t2597: F, t3596: F, t994: F, t2511: F, t3534: F, t2517: F, t1403: F, t7405: F, t2475: F, t7403: F, t3499: F, t7415: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9558 = t1428 * t2589;
    let t9561 = t3600 * t2574;
    let t9564 = t3596 * t2597;
    let t9565 = t9564 * t994;
    let t9568 = t3600 * t2589;
    let t9571 = t3534 * t2511;
    let t9573 = 0.16081979498692535067e2 * t2517 * t9571;
    let t9574 = t1403 * t7405;
    let t9575 = t9574 * t2475;
    let t9577 = 0.51726012919273400301e3 * t7403 * t9575;
    let t9579 = 4.0 * t7415 * t3499;
    (t9558, t9561, t9564, t9565, t9568, t9571, t9573, t9575, t9577, t9579)
}
