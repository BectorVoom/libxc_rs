//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 266/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk266<F: Float>(t1113: F, t384: F, t1047: F, t1053: F, t1075: F, t329: F, t334: F) -> (F, F, F, F, F) {
    let t1114 = t384 * t1113;
    let t1124 = F::new(0.1141e1) * t1047;
    let t1126 = F::new(0.2445e0) * t1053;
    let t1130 = F::new(0.12225e0) * t1075;
    let t1137 = t329 * t334;
    (t1114, t1124, t1126, t1130, t1137)
}
