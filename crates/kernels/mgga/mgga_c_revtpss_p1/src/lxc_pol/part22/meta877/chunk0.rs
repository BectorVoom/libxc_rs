//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3043/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3043<F: Float>(t14939: F, t233: F, t689: F, t869: F, t10069: F, t14588: F, t10518: F, t14606: F, t231: F, t2782: F, t2783: F, t51380: F) -> (F, F, F, F) {
    let t51505 = t689 * t869 * t233 * t14939;
    let t51507 = t10069 * t14588;
    let t51512 = t14606 * t10518;
    let t51519 = t2782 * t2783 * t51380 * t231;
    (t51505, t51507, t51512, t51519)
}
