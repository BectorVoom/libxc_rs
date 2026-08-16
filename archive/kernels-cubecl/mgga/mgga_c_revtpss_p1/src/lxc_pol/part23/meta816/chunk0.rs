//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2662/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2662<F: Float>(t11921: F, t15716: F, t19456: F, t247: F, t19696: F, t3168: F, t15830: F, t4817: F, t1063: F, t11986: F, t6100: F, t20054: F, t3106: F) -> (F, F, F, F, F) {
    let t65298 = t15716 * t247 * t11921 * t19456;
    let t65342 = t19696 * t3168;
    let t65347 = t15830 * t4817;
    let t65357 = t1063 * t247 * t11986 * t6100;
    let t65359 = t3106 * t20054;
    (t65298, t65342, t65347, t65357, t65359)
}
