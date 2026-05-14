//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 780/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk780<F: Float>(t2677: F, t7097: F, t639: F, t5103: F, t1004: F, t1678: F, t184: F, t199: F, t1022: F, t5212: F, t1816: F, t5211: F, t1044: F, t1811: F, t108: F, t210: F) -> (F, F, F, F, F, F) {
    let t7098 = t2677 * t7097;
    let t7100 = 4.0 / 27.0 * t639 * t7098;
    let t7101 = 8.0 / 135.0 * t5103;
    let t7102 = t1678 * t1004;
    let t7103 = t7102 * t184;
    let t7105 = 4.0 / 15.0 * t7103 * t199;
    let t7106 = t5212 * t1022;
    let t7107 = t7106 * t1816;
    let t7109 = 16.0 / 45.0 * t5211 * t7107;
    let t7110 = t5212 * t1044;
    let t7111 = t7110 * t1811;
    let t7113 = 16.0 / 45.0 * t5211 * t7111;
    let t7114 = t210 * t108;
    (t7100, t7101, t7105, t7109, t7113, t7114)
}
