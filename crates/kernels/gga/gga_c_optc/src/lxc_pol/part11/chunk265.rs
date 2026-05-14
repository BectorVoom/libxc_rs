//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 265/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk265<F: Float>(t339: F, t765: F, t792: F, t349: F, t346: F, t362: F, t857: F, t357: F, t355: F, t176: F, t352: F, t275: F, t282: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t962 = 1.0 / t339;
    let t966 = 0.19388333333333333333e1 * t765;
    let t968 = 0.12315e-2 * t792;
    let t972 = t349 * t349;
    let t973 = 1.0 / t972;
    let t974 = t346 * t973;
    let t975 = 0.72691666666666666667e3 * t765;
    let t977 = 0.78666666666666666667e2 * t792;
    let t992 = t857 * t362;
    let t993 = t357 * t992;
    let t995 = t355 * t993 / 6.0;
    let t996 = t176 * t352;
    let t997 = t275 * sigma0;
    let t998 = t997 * t282;
    let t999 = t996 * t998;
    (t962, t966, t968, t972, t973, t974, t975, t977, t993, t995, t996, t997, t998, t999)
}
