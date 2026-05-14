//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1113/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1113<F: Float>(t10030: F, t237: F, t9863: F, t9902: F, t9984: F, t154: F, t907: F, t9795: F, t178: F, t8358: F, t2364: F) -> (F, F, F, F) {
    let t10033 = t237 * (t9863 + t9902 + t9984 + t10030);
    let t10038 = t154 * t907 * t9795;
    let t10043 = t8358 * t178;
    let t10044 = t2364 * t10043;
    (t10033, t10038, t10043, t10044)
}
