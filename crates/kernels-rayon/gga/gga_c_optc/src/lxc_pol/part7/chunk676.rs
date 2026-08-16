//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 676/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk676(t6319: f64, t88: f64, t2041: f64, t538: f64, t6163: f64, t36: f64, t1872: f64, t539: f64, t2229: f64, t740: f64, t2234: f64, t2238: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6320 = t6319 * t88;
    let t6321 = 144.0_f64 * t6320;
    let t6322 = t538 * t2041;
    let t6323 = t6322 * t88;
    let t6324 = 240.0_f64 * t6323;
    let t6325 = 1.0_f64 / t6163;
    let t6326 = t36 * t6325;
    let t6328 = 120.0_f64 * t6326 * t88;
    let t6329 = t539 * t1872;
    let t6330 = 12.0_f64 * t6329;
    let t6332 = 7.0_f64 / 2.0_f64 * t2229 * t740;
    let t6333 = t2234 * t740;
    let t6335 = t2238 * t740;
    (t6321, t6322, t6324, t6325, t6326, t6328, t6330, t6332, t6333, t6335)
}
