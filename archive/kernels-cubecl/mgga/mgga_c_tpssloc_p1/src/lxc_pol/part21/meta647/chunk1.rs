//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2442/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2442<F: Float>(t3014: F, t4509: F, t42308: F, t974: F, t10224: F, t2999: F, t973: F, t2978: F, t698: F, t2981: F, t2402: F, t976: F) -> (F, F, F, F, F, F) {
    let t42846 = t4509 * t3014;
    let t42861 = t974 * t42308;
    let t42873 = t973 * t10224 * t2999;
    let t42875 = t698 * t2978;
    let t42877 = t973 * t42875 * t2981;
    let t42891 = t2402 * t976;
    (t42846, t42861, t42873, t42875, t42877, t42891)
}
