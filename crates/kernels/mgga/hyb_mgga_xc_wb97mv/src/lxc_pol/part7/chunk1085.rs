//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1085/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1085<F: Float>(t1428: F, t3596: F, t4376: F, t994: F, t4373: F, t2597: F, t4372: F, t3600: F, t4359: F, t7318: F, t4333: F, t975: F, t1416: F, t3563: F, t4349: F, t4346: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11437 = t1428 * t3596;
    let t11440 = t4376 * t994;
    let t11443 = t4373 * t994;
    let t11446 = t4372 * t2597;
    let t11447 = t11446 * t994;
    let t11450 = t3600 * t3596;
    let t11453 = t4359 * t7318;
    let t11454 = t11453 * t994;
    let t11461 = t4333 * t975;
    let t11464 = t1416 * t3563;
    let t11467 = t4349 * t975;
    let t11470 = t4346 * t975;
    (t11437, t11440, t11443, t11446, t11447, t11450, t11453, t11454, t11461, t11464, t11467, t11470)
}
