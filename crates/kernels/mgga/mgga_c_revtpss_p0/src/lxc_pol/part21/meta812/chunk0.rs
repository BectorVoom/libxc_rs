//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2969/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2969<F: Float>(t15648: F, t999: F, t1011: F, t1655: F, t2438: F, t1014: F, t4579: F, t697: F, t3252: F, t4574: F, t16020: F, t1062: F, t15887: F) -> (F, F, F, F, F, F) {
    let t54112 = t15648 * t999;
    let t54118 = t1011 * t2438 * t1655;
    let t54122 = t1011 * t697 * t1014 * t4579;
    let t54123 = t54122 / F::cast_from(216.0_f64);
    let t54126 = t1011 * t697 * t3252 * t4574;
    let t54127 = t54126 / F::cast_from(324.0_f64);
    let t54130 = t16020 * t999;
    let t54137 = t15887 * t1062;
    (t54112, t54118, t54123, t54127, t54130, t54137)
}
