//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 827/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk827<F: Float>(t6262: F, t871: F, t2295: F, t877: F, t6122: F, t890: F, t2256: F, t858: F, t2258: F, t870: F) -> (F, F, F, F, F) {
    let t6263 = t6262 * t871;
    let t6266 = t877 * t2295;
    let t6269 = t6122 * t890;
    let t6272 = t858 * t2256;
    let t6275 = t2258 * t870;
    (t6263, t6266, t6269, t6272, t6275)
}
