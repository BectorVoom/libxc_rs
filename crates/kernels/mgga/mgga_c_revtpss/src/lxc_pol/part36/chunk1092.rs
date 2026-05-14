//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1092/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1092<F: Float>(t2410: F, t11238: F, t196: F, t3800: F, t12625: F, t458: F, t13180: F, t493: F, t90: F, t29: F, t560: F, t9655: F, t1389: F, t268: F, t10115: F, t555: F) -> (F, F, F, F, F, F, F, F, F) {
    let t41153 = t2410 * t2410;
    let t41154 = 1.0 / t41153;
    let t42859 = 1.0 / t11238 / t196;
    let t44125 = t3800 * t3800;
    let t44126 = 1.0 / t44125;
    let t44841 = 1.0 / t12625 / t458;
    let t45551 = 1.0 / t13180 / t493;
    let t45970 = t90 * t90;
    let t45972 = t29 / t45970;
    let t46361 = 1.0 / t9655 / t560;
    let t46808 = t1389 * t268;
    let t47567 = t10115 * t555;
    (t41154, t42859, t44126, t44841, t45551, t45972, t46361, t46808, t47567)
}
