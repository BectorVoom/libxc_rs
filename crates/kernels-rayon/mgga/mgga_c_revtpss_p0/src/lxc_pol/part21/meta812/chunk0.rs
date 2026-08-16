//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2969/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2969(t15648: f64, t999: f64, t1011: f64, t1655: f64, t2438: f64, t1014: f64, t4579: f64, t697: f64, t3252: f64, t4574: f64, t16020: f64, t1062: f64, t15887: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54112 = t15648 * t999;
    let t54118 = t1011 * t2438 * t1655;
    let t54122 = t1011 * t697 * t1014 * t4579;
    let t54123 = t54122 / 216.0_f64;
    let t54126 = t1011 * t697 * t3252 * t4574;
    let t54127 = t54126 / 324.0_f64;
    let t54130 = t16020 * t999;
    let t54137 = t15887 * t1062;
    (t54112, t54118, t54123, t54127, t54130, t54137)
}
