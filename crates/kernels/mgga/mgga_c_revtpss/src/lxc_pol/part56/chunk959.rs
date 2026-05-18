//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 959/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk959<F: Float>(t1214: F, t494: F, t247: F, t3719: F, t2148: F, t3140: F, t1243: F, t479: F, t3089: F) -> (F, F, F, F, F) {
    let t33406 = t494 * t1214;
    let t33408 = t247 * t3719 * t33406;
    let t33411 = t2148 * t3140;
    let t33412 = t1243 * t479;
    let t33414 = t33411 * t33412 * t3089;
    (t33406, t33408, t33411, t33412, t33414)
}
