//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3119/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3119<F: Float>(t17170: F, t3153: F, t12744: F, t17350: F, t3781: F, t5219: F, t5330: F, t12916: F, t17743: F, t3718: F, t1469: F, t3588: F) -> (F, F, F, F, F) {
    let t57373 = t17170 * t3153;
    let t57378 = t12744 * t17350;
    let t57382 = t5219 * t3781 * t5330;
    let t57386 = t3718 * t12916 * t17743;
    let t57394 = t1469 * t3588;
    (t57373, t57378, t57382, t57386, t57394)
}
