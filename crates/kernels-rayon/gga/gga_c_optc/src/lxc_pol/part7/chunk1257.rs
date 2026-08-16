//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1257/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1257(t24599: f64, t331: f64, t7976: f64, t7979: f64, t7970: f64, t7973: f64, t3902: f64, t913: f64, t916: f64, t2712: f64, t8068: f64, t7965: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25969 = 0.5224665647534064904e-2_f64 * t331 * t24599;
    let t25970 = t7976 * t7979;
    let t25972 = t7970 * t7973;
    let t25975 = t913 * t3902 * t916;
    let t25977 = t2712 * t8068;
    let t25979 = t2712 * t7965;
    (t25969, t25970, t25972, t25975, t25977, t25979)
}
