//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 843/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk843(t2650: f64, t2701: f64, t2704: f64, t2797: f64, t7867: f64, t8058: f64, t8063: f64, t8066: f64, t8069: f64, t8072: f64, t8076: f64, t8078: f64, t8083: f64, t913: f64, t917: f64, t930: f64, t940: f64, t953: f64) -> f64 {
    let t8089 = 0.5848048239485271795e1_f64 * t940 * t8058 + 0.11360101276506094136e1_f64 * t913 * t8063 - 0.60587206808032502059e1_f64 * t8066 - 0.75734008510040627575e0_f64 * t8069 + 0.33322963744417876133e2_f64 * t8072 * t917 - 0.57954409931925052365e-1_f64 * t8076 + 0.17386322979577515709e0_f64 * t930 * t8078 + 0.16121825426676543132e0_f64 * t2704 * t2650 - 0.20152281783345678915e-1_f64 * t8083 + 0.30228422675018518374e-1_f64 * t953 * t7867 + 0.46363527945540041892e0_f64 * t2797 * t2701;
    t8089
}
