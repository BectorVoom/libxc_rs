//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1206/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1206<F: Float>(t24858: F, t2666: F, t5135: F, t259: F, t7564: F, t546: F, t565: F, t20861: F, t7258: F, t2719: F, t6212: F, t6211: F, t6475: F, t19790: F, t938: F, t19789: F, t20145: F) -> (F, F, F, F, F, F, F) {
    let t24859 = 0.38140175656238781678e1 * t24858;
    let t24860 = t5135 * t2666;
    let t24882 = t7564 * t259;
    let t24883 = t546 * t24882;
    let t24886 = t565 * t24882;
    let t24900 = t20861 * t7258;
    let t24901 = 0.19043987679069580388e-1 * t24900;
    let t24902 = t6212 * t2719;
    let t24904 = t6475 * t6211 * t24902;
    let t24905 = 0.19043987679069580388e-1 * t24904;
    let t24906 = t19790 * t938;
    let t24908 = t20145 * t19789 * t24906;
    (t24859, t24860, t24883, t24886, t24901, t24905, t24908)
}
