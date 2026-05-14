//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1349/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1349<F: Float>(t25162: F, t31593: F, t31597: F, t25165: F, t2665: F, t4973: F, t6317: F, t18123: F, t6318: F, t10409: F, t4965: F, t31589: F, t113190: F, t113374: F, t18997: F, t19002: F, t99529: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t126913 = t25162 * t31593;
    let t126914 = t126913 / 18.0;
    let t126915 = t25162 * t31597;
    let t126916 = t126915 / 27.0;
    let t126919 = t6317 * t2665 * t25165 * t4973;
    let t126923 = t6317 * t2665 * t6318 * t18123;
    let t126927 = t6317 * t10409 * t25165 * t4965;
    let t126929 = t25162 * t31589;
    let t126930 = t126929 / 9.0;
    let t126932 = t113190 * t113374 * t18997;
    let t126935 = t113190 * t99529 * t19002;
    (t126913, t126914, t126915, t126916, t126919, t126923, t126927, t126929, t126930, t126932, t126935)
}
