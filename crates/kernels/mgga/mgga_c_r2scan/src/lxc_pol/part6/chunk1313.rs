//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1313/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1313<F: Float>(t24176: F, t6462: F, t2531: F, t560: F, t6085: F, t6086: F, t2110: F, t22: F, t3436: F, t6: F, t6068: F, t923: F, t19883: F, t8082: F, t19872: F, t8085: F) -> (F, F, F, F, F) {
    let t24804 = t6462 * t24176;
    let t24805 = 0.86743646395112941037e-3 * t24804;
    let t24814 = t2531 * t560;
    let t24816 = t6085 * t6086 * t24814;
    let t24822 = t22 * t6 * t3436 * t2110 * t6068 * t923;
    let t24825 = t19883 * t8082;
    let t24827 = t19872 * t8085;
    (t24805, t24816, t24822, t24825, t24827)
}
