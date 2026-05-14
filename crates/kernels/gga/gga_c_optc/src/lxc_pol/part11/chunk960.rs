//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 960/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk960<F: Float>(t25836: F, t8206: F, t2769: F, t1659: F, t2746: F, t301: F, t327: F, t24565: F, t24599: F, t331: F, t8124: F, t3145: F, t9: F, t2849: F, t22: F, t8950: F, sigma0: F) -> (F, F, F, F, F, F, F, F) {
    let t25837 = t8206 * t25836;
    let t25877 = t2769 * t25836;
    let t25883 = t1659 * t25836;
    let t25939 = 1.0 / t2746 / t327 * t301;
    let t25940 = t25939 * t24565;
    let t25969 = 0.5224665647534064904e-2 * t331 * t24599;
    let t25981 = t8124 * sigma0;
    let t25982 = t25981 * t25836;
    let t26133 = t9 * t3145;
    let t26134 = t26133 * t2849;
    let t26140 = t22 * t8950;
    (t25837, t25877, t25883, t25940, t25969, t25982, t26134, t26140)
}
