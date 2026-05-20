//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2494/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2494<F: Float>(t49321: F, t1897: F, t40317: F, t10111: F, t22: F, t5759: F, t14188: F, t2439: F, t2777: F, t10073: F, t14129: F, t14159: F, t3964: F, t9285: F) -> (F, F, F, F, F, F) {
    let t49322 = F::cast_from(0.39029762157531132076e-1_f64) * t49321;
    let t49354 = t40317 * t1897;
    let t49361 = t10111 * t5759 * t22;
    let t49426 = t2439 * t2777 * t14188;
    let t49429 = t10073 * t14129;
    let t49432 = t3964 * t14159 * t9285;
    (t49322, t49354, t49361, t49426, t49429, t49432)
}
