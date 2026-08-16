//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1583/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1583<F: Float>(t1432: F, t22964: F, t686: F, t72: F, t14239: F, t22332: F, t10023: F, t22863: F, t14141: F, t23037: F, t22336: F, t13790: F, t6843: F) -> (F, F, F, F, F, F) {
    let t86374 = t1432 * t22964 * t72 * t686;
    let t86377 = t14239 * t22332;
    let t86381 = t10023 * t22863 * t72 * t686;
    let t86401 = t14141 * t23037 * t72 * t686;
    let t86411 = t14239 * t22336;
    let t86413 = t13790 * t6843;
    (t86374, t86377, t86381, t86401, t86411, t86413)
}
