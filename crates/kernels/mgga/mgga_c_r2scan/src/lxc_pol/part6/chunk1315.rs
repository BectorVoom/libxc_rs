//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1315/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1315<F: Float>(t22868: F, t24831: F, t6086: F, t2133: F, t2294: F, t7990: F, t20994: F, t2547: F, t20150: F, t7460: F, t113: F, t5054: F, t2526: F, t551: F, t566: F, t6343: F) -> (F, F, F, F, F, F) {
    let t24833 = t22868 * t6086 * t24831;
    let t24836 = t2133 * t2294 * t7990;
    let t24838 = t20994 * t2547;
    let t24839 = 0.12805040077930161442e1 * t24838;
    let t24840 = t20150 * t7460;
    let t24847 = t113 * t5054;
    let t24858 = t566 * t551 * t6343 * t2526;
    (t24833, t24836, t24839, t24840, t24847, t24858)
}
