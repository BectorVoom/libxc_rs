//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1142/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1142<F: Float>(t24599: F, t331: F, t7976: F, t7979: F, t7970: F, t7973: F, t3902: F, t913: F, t916: F, t2712: F, t8068: F, t7965: F, t8124: F, t25836: F, t2602: F, t7274: F, t930: F, sigma0: F) -> (F, F, F, F, F, F, F, F) {
    let t25969 = 0.5224665647534064904e-2 * t331 * t24599;
    let t25970 = t7976 * t7979;
    let t25972 = t7970 * t7973;
    let t25975 = t913 * t3902 * t916;
    let t25977 = t2712 * t8068;
    let t25979 = t2712 * t7965;
    let t25981 = t8124 * sigma0;
    let t25982 = t25981 * t25836;
    let t25996 = t930 * t7274 * t2602;
    (t25969, t25970, t25972, t25975, t25977, t25979, t25982, t25996)
}
