//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 653/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk653<F: Float>(t3103: F, t979: F, t452: F, t488: F, t103: F, t4495: F, t379: F, t1902: F, t4607: F, t8372: F, t920: F, t1903: F, t18: F, t942: F, t11902: F, t3200: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t16047 = t3103 * t979;
    let t16049 = t452 * t488 * t16047;
    let t16052 = t103 * t4495;
    let t16053 = t16052 * t379;
    let t16054 = t1902 * t16053;
    let t16057 = t8372 * t4607;
    let t16060 = t920 * t3103;
    let t16061 = t1903 * t16060;
    let t16062 = t1902 * t16061;
    let t16065 = t18 * t942;
    let t16066 = t1903 * t16065;
    let t16067 = t1902 * t16066;
    let t16070 = t11902 * t3200;
    (t16047, t16049, t16053, t16054, t16057, t16060, t16061, t16062, t16065, t16066, t16067, t16070)
}
