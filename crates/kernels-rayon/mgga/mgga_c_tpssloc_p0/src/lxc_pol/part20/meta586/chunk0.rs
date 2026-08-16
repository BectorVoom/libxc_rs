//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2156/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2156(t23508: f64, t43292: f64, t11013: f64, t225: f64, t10163: f64, t386: f64, t68: f64, t11008: f64, t3215: f64, t3399: f64, t3402: f64, t11176: f64, t300: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t43577 = t23508 * t43292;
    let t43599 = t11013 * t225;
    let t43603 = 1.0_f64 / t10163 / t386;
    let t43604 = t68 * t43603;
    let t43619 = t11008 * t225;
    let t43636 = t3215 * t3215;
    let t43637 = 1.0_f64 / t43636;
    let t43688 = t3399 * t3399;
    let t43689 = 1.0_f64 / t43688;
    let t43691 = t3402 * t3402;
    let t43692 = 1.0_f64 / t43691;
    let t43700 = t300 * t11176;
    (t43577, t43599, t43604, t43619, t43637, t43689, t43692, t43700)
}
