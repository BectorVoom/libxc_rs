//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 664/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk664<F: Float>(t289: F, t8817: F, t2408: F, t275: F, t1652: F, t2060: F, t739: F, t2124: F, t558: F, t884: F, t321: F, t615: F) -> (F, F, F, F, F, F, F) {
    let t8818 = t289 * t8817;
    let t8820 = t275 * t2408;
    let t8821 = t2060 * t1652;
    let t8822 = t739 * t8821;
    let t8824 = t2124 * t558;
    let t8825 = t884 * t8824;
    let t8829 = t615 * t321;
    (t8818, t8820, t8821, t8822, t8824, t8825, t8829)
}
