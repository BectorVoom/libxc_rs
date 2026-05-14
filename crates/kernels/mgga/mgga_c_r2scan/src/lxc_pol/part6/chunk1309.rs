//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1309/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1309<F: Float>(t24732: F, t2670: F, t6504: F, t20868: F, t924: F, t2155: F, t24070: F, t6063: F, t24059: F, t8077: F, t2252: F, t921: F, t6085: F, t6086: F, t19890: F, t8081: F) -> (F, F, F, F, F, F, F) {
    let t24733 = 0.12713391885412927226e1 * t24732;
    let t24734 = t2670 * t6504;
    let t24735 = 0.38140175656238781678e1 * t24734;
    let t24742 = t20868 * t924;
    let t24745 = t2155 * t6063 * t24070;
    let t24748 = t2155 * t8077 * t24059;
    let t24750 = t921 * t2252;
    let t24752 = t6085 * t6086 * t24750;
    let t24755 = t6085 * t19890 * t8081;
    (t24733, t24735, t24742, t24745, t24748, t24752, t24755)
}
