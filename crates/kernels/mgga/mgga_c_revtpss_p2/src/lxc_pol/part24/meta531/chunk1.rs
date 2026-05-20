//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1568/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1568<F: Float>(t22970: F, t686: F, t72: F, t9680: F, t22453: F, t49471: F, t1358: F, t212: F, t22964: F, t689: F, t13848: F, t22893: F, t47274: F, t9816: F) -> (F, F, F, F) {
    let t85480 = t9680 * t22970 * t72 * t686;
    let t85484 = t49471 * t22453;
    let t85509 = t689 * t212 * t22964 * t1358;
    let t85514 = t9816 * t47274 * t13848 * t22893;
    (t85480, t85484, t85509, t85514)
}
