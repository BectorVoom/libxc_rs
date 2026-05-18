//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 415/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk415<F: Float>(t1513: F, t1515: F, t1243: F, t486: F, t48: F, t53: F, t118: F, t119: F, t120: F, t331: F, t156: F, t497: F) -> (F, F, F, F, F, F) {
    let t1516 = t1513 * t1515;
    let t1519 = F::new(0.64956111111111111111e0) * t486 * t1243;
    let t1523 = F::new(1.0) / t48;
    let t1528 = F::new(1.0) / t53;
    let t1540 = t118 * t119 * t331 * t120 / F::new(9.0);
    let t1541 = t156 * t497;
    (t1516, t1519, t1523, t1528, t1540, t1541)
}
