//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1312/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1312(t24863: f64, t24864: f64, t30189: f64, t30270: f64, t49378: f64, t49381: f64, t49385: f64, t49387: f64, t49393: f64, t56988: f64, t56991: f64, t56994: f64, t56997: f64, t56999: f64) -> f64 {
    let t57416 = -0.375102e1_f64 * t56988 + 0.83356e0_f64 * t56991 + 0.125034e1_f64 * t56994 + 0.12349037037037037037e1_f64 * t30189 + t24863 + t24864 - 0.94674375e0_f64 * t56997 + 0.1262325e1_f64 * t56999 + 0.12349037037037037037e0_f64 * t49378 + 0.27785333333333333333e0_f64 * t49381 + 0.21424148148148148148e1_f64 * t30270 - 0.27545333333333333332e1_f64 * t49385 + 0.41318e1_f64 * t49387 + 0.68863333333333333332e0_f64 * t49393;
    t57416
}
