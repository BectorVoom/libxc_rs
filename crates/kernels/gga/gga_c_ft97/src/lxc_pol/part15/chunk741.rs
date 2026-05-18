//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 741/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk741<F: Float>(t144: F, t20893: F, t1053: F, t4724: F, t9439: F, t4805: F, t2179: F, t3578: F, t167: F, t20655: F, t574: F, t1060: F, t4714: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t20894 = t144 * t20893;
    let t20897 = t4724 * t1053;
    let t20898 = t9439 * t20897;
    let t20899 = t144 * t20898;
    let t20902 = t1053 * t4805;
    let t20903 = t2179 * t20902;
    let t20904 = t144 * t20903;
    let t20908 = t3578 * t4805;
    let t20909 = t144 * t20908;
    let t20912 = t574 * t167 * t20655;
    let t20916 = t574 * t1060 * t4714;
    (t20894, t20897, t20898, t20899, t20902, t20903, t20904, t20908, t20909, t20912, t20916)
}
