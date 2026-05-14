//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1147/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1147<F: Float>(t13947: F, t4793: F, t16729: F, t3681: F, t24287: F, t24288: F, t30189: F, t30270: F, t49378: F, t49381: F, t49385: F, t49387: F, t49393: F, t56988: F, t56991: F, t56994: F) -> (F, F, F) {
    let t56997 = t13947 * t4793;
    let t56999 = t3681 * t16729;
    let t57007 = -0.295764e1 * t56988 + 0.65725333333333333332e0 * t56991 + 0.98587999999999999999e0 * t56994 + 0.97370864197530864199e0 * t30189 + t24287 + t24288 - 0.46074375e0 * t56997 + 0.614325e0 * t56999 + 0.97370864197530864196e-1 * t49378 + 0.21908444444444444444e0 * t49381 + 0.12401580246913580247e1 * t30270 - 0.15944888888888888889e1 * t49385 + 0.23917333333333333333e1 * t49387 + 0.39862222222222222223e0 * t49393;
    (t56997, t56999, t57007)
}
