//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1877/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1877<F: Float>(t13126: F, t460: F, t13043: F, t487: F, t12051: F, t471: F, t3727: F, t473: F, t1214: F, t11239: F, t3596: F) -> (F, F, F, F, F, F, F) {
    let t13127 = t460 * t13126;
    let t13128 = t487 * t13043;
    let t13129 = t12051 * t471;
    let t13130 = t13128 * t13129;
    let t13133 = t473 * t3727;
    let t13134 = t13133 * t1214;
    let t13141 = t11239 * t3596;
    (t13127, t13128, t13129, t13130, t13133, t13134, t13141)
}
