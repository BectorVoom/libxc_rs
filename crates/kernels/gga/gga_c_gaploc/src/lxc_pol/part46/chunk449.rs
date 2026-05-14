//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 449/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk449<F: Float>(t2021: F, t6110: F, t1858: F, t935: F, t1890: F, t7291: F, t739: F, t7068: F, t1: F, t2530: F, t106: F, t316: F, t325: F, t883: F, t900: F, t6574: F, t823: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7630 = t2021 * t6110;
    let t7634 = t1858 * t935;
    let t7659 = t1890 * t7291;
    let t7667 = t739 * t7291;
    let t7671 = t739 * t7068;
    let t7710 = t2530 * t1;
    let t7711 = t7710 * t106;
    let t7712 = t7711 * t316;
    let t7784 = t883 * t325;
    let t7785 = t900 * t7784;
    let t7802 = t823 * t6574;
    (t7630, t7634, t7659, t7667, t7671, t7712, t7784, t7785, t7802)
}
