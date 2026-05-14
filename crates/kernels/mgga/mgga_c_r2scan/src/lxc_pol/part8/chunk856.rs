//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 856/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk856<F: Float>(t4938: F, t889: F, t1361: F, t35: F, t4948: F, t893: F, t1368: F, t5021: F, t5872: F, t5874: F, t1509: F, t898: F, t41: F, t1531: F, t2463: F, t2: F, t2483: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6976 = t4938 * t889;
    let t6979 = t1361 * t35;
    let t6991 = t4948 * t893;
    let t6994 = t1368 * t35;
    let t7025 = 4.0 * t5021;
    let t7026 = 1584.0 * t5872;
    let t7027 = 1872.0 * t5874;
    let t7030 = t898 * t1509;
    let t7031 = t41 * t7030;
    let t7032 = t2463 * t1531;
    let t7034 = t2483 * t2;
    (t6976, t6979, t6991, t6994, t7025, t7026, t7027, t7030, t7031, t7032, t7034)
}
