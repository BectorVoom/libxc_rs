//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1234/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1234(t29330: f64, t29335: f64, t29341: f64, t38368: f64, t22581: f64, t22593: f64, t22697: f64, t22703: f64, t22708: f64, t22711: f64, t22716: f64, t22719: f64) -> (f64, f64, f64, f64, f64) {
    let t56294 = 0.23392893589820816284e1_f64 * t29330;
    let t56295 = 0.1926377843805564792e1_f64 * t29335;
    let t56296 = 0.65061485296689145286e-1_f64 * t29341;
    let t56297 = 48.0_f64 * t38368;
    let t56298 = t22581 - t22593 - t56294 + t22697 + t22703 + t22708 - t22711 - t22716 - t22719 + t56295 + t56296 - t56297;
    (t56294, t56295, t56296, t56297, t56298)
}
