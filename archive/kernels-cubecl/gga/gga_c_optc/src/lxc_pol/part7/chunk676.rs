//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 676/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk676<F: Float>(t6319: F, t88: F, t2041: F, t538: F, t6163: F, t36: F, t1872: F, t539: F, t2229: F, t740: F, t2234: F, t2238: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6320 = t6319 * t88;
    let t6321 = F::cast_from(144.0_f64) * t6320;
    let t6322 = t538 * t2041;
    let t6323 = t6322 * t88;
    let t6324 = F::cast_from(240.0_f64) * t6323;
    let t6325 = F::cast_from(1.0_f64) / t6163;
    let t6326 = t36 * t6325;
    let t6328 = F::cast_from(120.0_f64) * t6326 * t88;
    let t6329 = t539 * t1872;
    let t6330 = F::cast_from(12.0_f64) * t6329;
    let t6332 = F::cast_from(7.0_f64) / F::cast_from(2.0_f64) * t2229 * t740;
    let t6333 = t2234 * t740;
    let t6335 = t2238 * t740;
    (t6321, t6322, t6324, t6325, t6326, t6328, t6330, t6332, t6333, t6335)
}
