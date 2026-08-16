//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 532/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk532(t1020: f64, t1038: f64, t1041: f64, t1607: f64, t1612: f64, t1618: f64, t1622: f64, t378: f64, t973: f64, t997: f64) -> f64 {
    let t1625 = t997 + t973 * t1607 / 288.0_f64 + t1612 * t378 / 3072.0_f64 + t1020 * t1618 / 3072.0_f64 + t1038 + t1041 * t1622 / 4608.0_f64;
    t1625
}
