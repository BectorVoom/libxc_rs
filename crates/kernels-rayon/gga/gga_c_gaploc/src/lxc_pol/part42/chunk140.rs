//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 140/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk140(t241: f64, t629: f64, t367: f64, t46: f64, t372: f64, t374: f64, t231: f64, t242: f64, t337: f64, t359: f64, t4: f64, t55: f64, t624: f64, t79: f64) -> f64 {
    let t630 = 1.0_f64 / t241;
    let t631 = t629 * t630;
    let t637 = t46 * t367;
    let t638 = t372 * t374;
    let t642 = t231 * (0.53236443333333333332e-3_f64 * t4 * t79 * t242 + 1.0_f64 * t624 * t631 - t337 - t359 + 0.18311555036753159941e-3_f64 * t4 * t79 * t55 + 0.58482233974552040708e0_f64 * t637 * t638);
    t642
}
