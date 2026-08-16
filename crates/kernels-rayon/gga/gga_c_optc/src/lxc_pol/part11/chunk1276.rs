//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1276/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1276(t13947: f64, t4793: f64, t16729: f64, t3681: f64, t24287: f64, t24288: f64, t30189: f64, t30270: f64, t49378: f64, t49381: f64, t49385: f64, t49387: f64, t49393: f64, t56988: f64, t56991: f64, t56994: f64) -> (f64, f64, f64) {
    let t56997 = t13947 * t4793;
    let t56999 = t3681 * t16729;
    let t57007 = -0.295764e1_f64 * t56988 + 0.65725333333333333332e0_f64 * t56991 + 0.98587999999999999999e0_f64 * t56994 + 0.97370864197530864199e0_f64 * t30189 + t24287 + t24288 - 0.46074375e0_f64 * t56997 + 0.614325e0_f64 * t56999 + 0.97370864197530864196e-1_f64 * t49378 + 0.21908444444444444444e0_f64 * t49381 + 0.12401580246913580247e1_f64 * t30270 - 0.15944888888888888889e1_f64 * t49385 + 0.23917333333333333333e1_f64 * t49387 + 0.39862222222222222223e0_f64 * t49393;
    (t56997, t56999, t57007)
}
