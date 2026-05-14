//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 410/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk410<F: Float>(t1235: F, t103: F, t1251: F, t1: F, t120: F, t485: F, t119: F, t155: F, t481: F, t1243: F, t486: F, t48: F, t53: F, t118: F, t331: F, t156: F, t497: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t1508 = param_hyb_omega_0 * t1235;
    let t1509 = t1508 * t103;
    let t1511 = 0.32478055555555555555e0 * t1509 * t1251;
    let t1513 = t485 * t120 * t1;
    let t1515 = t119 * t155 * t481;
    let t1516 = t1513 * t1515;
    let t1519 = 0.64956111111111111111e0 * t486 * t1243;
    let t1523 = 1.0 / t48;
    let t1528 = 1.0 / t53;
    let t1540 = t118 * t119 * t331 * t120 / 9.0;
    let t1541 = t156 * t497;
    (t1508, t1509, t1511, t1513, t1515, t1516, t1519, t1523, t1528, t1540, t1541)
}
