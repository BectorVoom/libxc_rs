//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1020/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1020(t28: f64, t1081: f64, t3711: f64, t11122: f64, t12000: f64, t12001: f64, t1302: f64, t3231: f64, t11997: f64, zeta_threshold: f64) -> f64 {
    let t29 = t28 <= zeta_threshold;
    let t12004 = t3711 * t1081;
    let t12010 = piecewise3(t29, 0.0_f64, 8.0_f64 / 27.0_f64 * t12000 * t12001 - 2.0_f64 / 3.0_f64 * t12004 * t3231 + 2.0_f64 / 3.0_f64 * t1302 * t11122);
    let t12012 = t11997 / 2.0_f64 + t12010 / 2.0_f64;
    t12012
}
