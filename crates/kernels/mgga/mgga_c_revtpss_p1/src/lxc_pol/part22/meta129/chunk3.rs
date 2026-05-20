//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 867/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk867<F: Float>(t3092: F, t3096: F, t1066: F, t2858: F, t247: F, t1052: F, t369: F, t361: F) -> (F, F, F) {
    let t3097 = t3092 * t3096;
    let t3100 = t1066 * t2858;
    let t3101 = t247 * t3100;
    let t3104 = t1052 * t369;
    let t3105 = t361 * t3104;
    (t3097, t3101, t3105)
}
