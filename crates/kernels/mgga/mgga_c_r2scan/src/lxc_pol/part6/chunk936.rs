//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 936/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk936<F: Float>(t2065: F, t2271: F, t4721: F, t4791: F, t4794: F, t4798: F, t4806: F, t4964: F, t4967: F, t4969: F, t4972: F, t4975: F, t4977: F, t4979: F, t4981: F, t4984: F, t6014: F, t6017: F, t6021: F, t881: F) -> (F, F) {
    let t6794 = t2271 * t2065;
    let t6796 = -t4721 + t4964 - t4967 - t4969 - t4972 + t4975 + t4977 + t4979 - t4981 + t4984 - t4791 + t4794 + t4798 - t4806 - 0.7089e1 * t881 * t6021 - 0.2363e1 * t881 * t6014 - 0.7089e1 * t881 * t6017 - 0.7089e1 * t6794;
    (t6794, t6796)
}
