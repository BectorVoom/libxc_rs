//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1006/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1006<F: Float>(t1173: F, t6838: F, t193: F, t1131: F, t6008: F, t263: F, t5053: F, t27882: F, t6752: F, t1403: F, t1427: F, t24213: F, t27916: F, t27927: F, t27930: F, t27936: F, t28015: F, t30862: F, t30867: F, t30871: F, t30875: F, t30879: F, t30883: F, t30896: F, t6002: F, t6745: F, t6749: F, t6840: F, t6844: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t30899 = t6838 * t1173;
    let t30900 = t193 * t30899;
    let t30904 = t1173 * t1131;
    let t30905 = t6008 * t30904;
    let t30906 = t193 * t30905;
    let t30909 = t263 * t5053;
    let t30910 = t6008 * t30909;
    let t30911 = t193 * t30910;
    let t30914 = t27882 * t6752;
    let t30915 = t193 * t30914;
    let t30918 = t1403 * t30862 / 6.0 - t27916 / 9.0 + t1403 * t30867 / 6.0 - t6002 * t30871 / 18.0 - t6002 * t30875 / 27.0 - t6002 * t30879 / 9.0 - t6002 * t30883 / 9.0 - t28015 * t6749 / 9.0 - t27927 / 9.0 - t27930 / 9.0 + t6745 * t6844 / 3.0 + t6745 * t6840 / 3.0 + t30896 * t1427 / 6.0 + t1403 * t30900 / 3.0 + t24213 + t27936 / 27.0 - 2.0 / 3.0 * t1403 * t30906 - t1403 * t30911 / 3.0 - 2.0 / 3.0 * t1403 * t30915;
    (t30899, t30900, t30904, t30905, t30906, t30909, t30910, t30911, t30914, t30915, t30918)
}
