//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 843/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk843<F: Float>(t1816: F, t7106: F, t5211: F, t1044: F, t5212: F, t1811: F, t108: F, t210: F, t267: F, t1791: F, t641: F, t1018: F, t1672: F) -> (F, F, F, F, F) {
    let t7107 = t7106 * t1816;
    let t7109 = F::new(16.0) / F::new(45.0) * t5211 * t7107;
    let t7110 = t5212 * t1044;
    let t7111 = t7110 * t1811;
    let t7113 = F::new(16.0) / F::new(45.0) * t5211 * t7111;
    let t7114 = t210 * t108;
    let t7115 = t7114 * t267;
    let t7116 = t641 * t1791;
    let t7117 = t7116 * t1044;
    let t7118 = t7117 * t1816;
    let t7120 = F::new(16.0) / F::new(45.0) * t7115 * t7118;
    let t7121 = t1672 * t1018;
    (t7109, t7113, t7115, t7120, t7121)
}
