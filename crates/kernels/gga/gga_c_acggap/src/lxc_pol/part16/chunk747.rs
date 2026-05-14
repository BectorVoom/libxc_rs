//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 747/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk747<F: Float>(t8444: F, t9022: F, t9057: F, t9079: F, t105: F, t469: F, t301: F, t560: F, t2541: F, t566: F, t95: F, t3952: F, t624: F) -> (F, F, F, F, F, F, F) {
    let t9081 = t8444 + t9022 + t9057 + t9079;
    let t9082 = t105 * t9081;
    let t9083 = t9082 * t469;
    let t9089 = t560 * t301;
    let t9090 = t2541 * t9089;
    let t9096 = t566 * t95 * t105;
    let t9097 = t624 * t3952;
    (t9081, t9082, t9083, t9089, t9090, t9096, t9097)
}
