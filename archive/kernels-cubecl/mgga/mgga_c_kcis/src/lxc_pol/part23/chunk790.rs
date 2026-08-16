//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 790/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk790<F: Float>(t12140: F, t3980: F, t1368: F, t25: F, t4002: F, t493: F, t1377: F, t3970: F) -> (F, F, F) {
    let t12141 = t12140 * t3980;
    let t12142 = t1368 * t12141;
    let t12144 = t25 * t4002;
    let t12145 = t493 * t12144;
    let t12147 = t3970 * t1377;
    (t12142, t12145, t12147)
}
