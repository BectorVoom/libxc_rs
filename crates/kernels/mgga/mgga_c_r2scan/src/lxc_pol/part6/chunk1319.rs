//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1319/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1319<F: Float>(t20861: F, t7258: F, t2719: F, t6212: F, t6211: F, t6475: F, t19790: F, t938: F, t19789: F, t20145: F, t20864: F, t7262: F, t2526: F, t6480: F, t910: F, t19787: F) -> (F, F, F, F, F, F) {
    let t24900 = t20861 * t7258;
    let t24901 = 0.19043987679069580388e-1 * t24900;
    let t24902 = t6212 * t2719;
    let t24904 = t6475 * t6211 * t24902;
    let t24905 = 0.19043987679069580388e-1 * t24904;
    let t24906 = t19790 * t938;
    let t24908 = t20145 * t19789 * t24906;
    let t24910 = t20864 * t7262;
    let t24911 = 0.57131963037208741166e-1 * t24910;
    let t24912 = t6212 * t2526;
    let t24914 = t6480 * t6211 * t24912;
    let t24915 = 0.57131963037208741166e-1 * t24914;
    let t24916 = t19790 * t910;
    let t24918 = t19787 * t19789 * t24916;
    (t24901, t24905, t24908, t24911, t24915, t24918)
}
