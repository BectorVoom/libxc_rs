//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 745/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk745<F: Float>(t4987: F, t1380: F, t453: F, t4811: F, t234: F) -> (F, F, F, F) {
    let t4988 = 0.51947577317044391276e2 * t4987;
    let t4990 = t1380 * t4811 * t453;
    let t4991 = t234 * t4990;
    let t4992 = 0.35089341735807877242e1 * t4991;
    (t4988, t4990, t4991, t4992)
}
