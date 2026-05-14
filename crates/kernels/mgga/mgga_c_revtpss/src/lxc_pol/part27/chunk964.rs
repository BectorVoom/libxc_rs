//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 964/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk964<F: Float>(t13043: F, t487: F, t12051: F, t471: F, t3727: F, t473: F, t1214: F, t11239: F, t3596: F, t460: F, t3603: F, t13038: F, t13045: F, t13111: F, t3783: F, t3568: F, t3759: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t13128 = t487 * t13043;
    let t13129 = t12051 * t471;
    let t13130 = t13128 * t13129;
    let t13133 = t473 * t3727;
    let t13134 = t13133 * t1214;
    let t13141 = t11239 * t3596;
    let t13142 = t460 * t13141;
    let t13143 = t12051 * t3603;
    let t13144 = t13128 * t13143;
    let t13147 = t11239 * t13038;
    let t13148 = t460 * t13147;
    let t13149 = t12051 * t13045;
    let t13150 = t13128 * t13149;
    let t13153 = t13111 * t3783;
    let t13156 = t3759 * t3568;
    (t13129, t13130, t13134, t13142, t13143, t13144, t13148, t13149, t13150, t13153, t13156)
}
