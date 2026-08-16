//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2186/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2186<F: Float>(t11661: F, t13969: F, t3506: F, t11721: F, t3493: F, t11858: F, t1226: F, t3030: F, t3481: F, t3032: F, t3505: F, t3514: F) -> (F, F, F, F, F, F) {
    let t44904 = t3506 * t13969 * t11661;
    let t44906 = t11721 * t3493;
    let t44918 = t11858 * t1226;
    let t44927 = t3481 * t3030;
    let t44928 = t44927 * t3032;
    let t44929 = t44928 * t3505;
    let t44932 = t44928 * t3514;
    (t44904, t44906, t44918, t44927, t44929, t44932)
}
