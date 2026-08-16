//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 922/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk922<F: Float>(t4181: F, t5268: F, t1042: F, t1032: F, t1770: F, t1246: F) -> (F, F, F, F) {
    let t5269 = t5268 * t4181;
    let t5270 = t1042 * t5269;
    let t5273 = t1770 * t1032;
    let t5274 = t5273 * t1246;
    (t5269, t5270, t5273, t5274)
}
