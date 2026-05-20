//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2550/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2550<F: Float>(t1045: F, t20089: F, t3117: F, t1651: F, t2857: F, t4181: F, t3092: F, t2852: F) -> (F, F, F, F, F, F) {
    let t20090 = t20089 * t1045;
    let t20091 = t3117 * t20090;
    let t20094 = t1651 * t2857;
    let t20095 = t20094 * t4181;
    let t20096 = t3092 * t20095;
    let t20099 = t1651 * t2852;
    (t20090, t20091, t20094, t20095, t20096, t20099)
}
