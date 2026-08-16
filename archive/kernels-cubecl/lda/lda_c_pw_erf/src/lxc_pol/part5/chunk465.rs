//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 465/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk465<F: Float>(t1901: F, t242: F, t632: F, t781: F, t168: F, t635: F, t861: F, t1904: F, t247: F, t251: F, t652: F, t850: F) -> (F, F, F, F, F, F) {
    let t2240 = t1901 * t242;
    let t2244 = t781 * t632;
    let t2249 = t168 * t635 * t861;
    let t2252 = t1904 * t247;
    let t2253 = t2252 * t251;
    let t2256 = t850 * t652;
    (t2240, t2244, t2249, t2252, t2253, t2256)
}
