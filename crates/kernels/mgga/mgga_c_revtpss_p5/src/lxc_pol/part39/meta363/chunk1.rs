//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1269/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1269<F: Float>(t15135: F, t2908: F, t141: F, t11341: F, t15140: F, t15145: F, t930: F, t15149: F, t1593: F, t2435: F) -> (F, F, F, F, F) {
    let t15177 = t2908 * t15135;
    let t15178 = t141 * t15177;
    let t15180 = t11341 * t15140;
    let t15181 = t141 * t15180;
    let t15183 = t930 * t15145;
    let t15184 = t141 * t15183;
    let t15186 = t930 * t15149;
    let t15187 = t141 * t15186;
    let t15189 = t2435 * t1593;
    (t15178, t15181, t15184, t15187, t15189)
}
