//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1177/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1177<F: Float>(t1819: F, t1836: F, t1966: F, t2005: F, t21097: F, t21102: F, t21104: F, t21107: F, t21110: F, t21136: F, t21176: F, t390: F, t5318: F, t5435: F, t5439: F, t5569: F, t5627: F, t5628: F, t5640: F, t5782: F, t5787: F, t5801: F, t748: F) -> (F,) {
    let t21461 = t21097 - t21102 - 0.39036892681086263432e0 * t390 * t748 * t5435 + 0.11558335953042377059e2 * t390 * t1836 * t5439 - t21104 - 0.42514644538609193172e3 * t390 * t5627 * t5787 + t21107 - t21110 - 0.11407595979765752407e3 * t390 * t1819 * t5569 + 0.25508786723165515904e4 * t2005 * t1966 * t5628 + 0.77055573020282513724e1 * t5801 * t5318 - t21136 + 0.39654301768696105266e2 * t390 * t5640 * t5782 + t21176;
    (t21461,)
}
