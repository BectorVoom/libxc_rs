//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 524/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk524<F: Float>(t2847: F, t1388: F, t224: F, t1: F, t1378: F, t283: F, t2894: F, t1390: F, t229: F, t276: F, t40: F, t483: F, t803: F, t119: F, t1603: F, t1308: F, t872: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4044 = 32.0 * t2847;
    let t4045 = t224 * t1388;
    let t4046 = 8.0 * t4045;
    let t4047 = t1378 * t1;
    let t4048 = t4047 * t283;
    let t4049 = 0.36622894612013090108e-3 * t4048;
    let t4050 = 12.0 * t2894;
    let t4057 = t229 * t1390;
    let t4059 = t1378 * t276;
    let t4060 = t40 * t4059;
    let t4061 = 2.0 * t4060;
    let t4062 = t229 * t1388;
    let t4063 = 8.0 * t4062;
    let t4068 = t483 * t803;
    let t4069 = t40 * t4068;
    let t4103 = t119 * t1603;
    let t4107 = 0.13170898365871023197e1 * t1308 * t872;
    (t4044, t4046, t4049, t4050, t4057, t4061, t4063, t4069, t4103, t4107)
}
