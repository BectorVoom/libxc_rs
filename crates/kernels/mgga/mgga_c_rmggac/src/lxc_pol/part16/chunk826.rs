//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 826/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk826<F: Float>(t1965: F, t9824: F, t1969: F, t1973: F, t1756: F, t1971: F, t495: F, t515: F, t7230: F, t10014: F, t35637: F, t34884: F, t9990: F, t10095: F, t16156: F, t10082: F, t333: F, t3351: F, t511: F, t7248: F) -> (F, F, F, F, F, F) {
    let t45475 = t9824 * t1965;
    let t45476 = t45475 * t1969;
    let t45477 = t45476 * t1973;
    let t45482 = t7230 * t1971 * t515 * t1756 * t495;
    let t45484 = t35637 * t10014;
    let t45486 = t34884 * t9990;
    let t45488 = t16156 * t10095;
    let t45493 = t3351 * t7248 * t511 * t10082 * t333;
    (t45477, t45482, t45484, t45486, t45488, t45493)
}
