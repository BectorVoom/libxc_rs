//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1297/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1297(t23926: f64, t23927: f64, t30189: f64, t30270: f64, t49378: f64, t49381: f64, t49385: f64, t49387: f64, t49393: f64, t56988: f64, t56991: f64, t56994: f64, t56997: f64, t56999: f64) -> f64 {
    let t57148 = -0.298026e1_f64 * t56988 + 0.66228e0_f64 * t56991 + 0.99342e0_f64 * t56994 + 0.98115555555555555556e0_f64 * t30189 + t23926 + t23927 - 0.247573125e0_f64 * t56997 + 0.3300975e0_f64 * t56999 + 0.98115555555555555555e-1_f64 * t49378 + 0.22076e0_f64 * t49381 + 0.12524296296296296297e1_f64 * t30270 - 0.16102666666666666667e1_f64 * t49385 + 0.24154e1_f64 * t49387 + 0.40256666666666666668e0_f64 * t49393;
    t57148
}
