//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 930/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk930<F: Float>(t10075: F, t10121: F, t406: F, t154: F, t3757: F, t6431: F, t385: F, t2347: F, t3730: F, t1220: F, t3171: F, t2099: F, t3876: F) -> (F, F, F, F, F, F, F, F) {
    let t10122 = t10075 * t10121;
    let t10123 = t406 * t10122;
    let t10131 = t154 * t6431 * t3757;
    let t10132 = t385 * t10131;
    let t10135 = t154 * t2347 * t3730;
    let t10136 = t385 * t10135;
    let t10138 = t1220 * t3171;
    let t10140 = t2099 * t3876;
    (t10122, t10123, t10131, t10132, t10135, t10136, t10138, t10140)
}
