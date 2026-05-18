//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 761/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk761<F: Float>(t1341: F, t357: F, t638: F, t7310: F, t7254: F, t7364: F, t7243: F, t1973: F, t1965: F, t7942: F, t1969: F, t1987: F, t34881: F) -> (F, F, F, F, F, F) {
    let t35633 = t638 * t7310 * t357 * t1341;
    let t35637 = t7254 * t7364;
    let t35654 = t7254 * t7243;
    let t35655 = t35654 * t1973;
    let t35657 = t7942 * t1965;
    let t35658 = t35657 * t1969;
    let t35665 = t34881 * t1987;
    (t35633, t35637, t35654, t35655, t35658, t35665)
}
