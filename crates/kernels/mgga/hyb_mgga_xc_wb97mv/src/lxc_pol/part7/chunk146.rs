//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 146/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk146<F: Float>(t435: F, t14: F, t237: F, t240: F, t438: F) -> (F, F, F, F, F) {
    let t441 = pow_3_2(t435);
    let t444 = t237 * t14 * t240;
    let t446 = 0.379785e1 * t438 + 0.8969e0 * t435 + 0.204775e0 * t441 + 0.123235e0 * t444;
    let t449 = 1.0 + 0.16081979498692535067e2 / t446;
    let t450 = f64::ln(t449);
    (t441, t444, t446, t449, t450)
}
