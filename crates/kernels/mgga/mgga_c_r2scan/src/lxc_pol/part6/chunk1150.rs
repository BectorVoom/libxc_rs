//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1150/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1150<F: Float>(t5115: F, t5147: F, t5148: F, t122: F, t625: F, t6412: F, t5149: F, t2233: F, t6512: F, t18783: F, t239: F, t5: F, t4715: F, t753: F, t1398: F, t2040: F) -> (F, F, F, F, F, F, F) {
    let t21025 = t5147 * t5148 * t5115;
    let t21028 = t625 * t6412 * t122;
    let t21029 = t21028 * t5149;
    let t21032 = t6512 * t2233;
    let t21036 = 1400.0 / 81.0 * t5 * t18783 * t239;
    let t21038 = t5 * t4715 * t753;
    let t21041 = t5 * t1398 * t2040;
    (t21025, t21028, t21029, t21032, t21036, t21038, t21041)
}
