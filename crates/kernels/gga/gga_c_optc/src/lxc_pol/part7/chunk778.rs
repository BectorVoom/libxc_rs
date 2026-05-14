//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 778/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk778<F: Float>(t7670: F, t799: F, t2416: F, t2409: F, t800: F, t2373: F, t2472: F, t7342: F, t837: F, t845: F, t7523: F, t7525: F, t7527: F, t7529: F, t7531: F, t7535: F, t7538: F, t7541: F, t7544: F, t7547: F, t7550: F) -> (F, F, F, F, F, F, F) {
    let t7689 = t7670 * t799;
    let t7691 = 6.0 * t2416 * t7689;
    let t7692 = t800 * t2409;
    let t7694 = 6.0 * t2373 * t7692;
    let t7696 = t2472 * t7342 * t837;
    let t7698 = 0.35089340384731224426e1 * t845 * t7696;
    let t7699 = 0.53272592592592592592e-1 * t7523;
    let t7710 = -t7699 - 0.2283111111111111111e-1 * t7525 + 0.11415555555555555555e-1 * t7527 - 0.34246666666666666665e-1 * t7529 + 0.17123333333333333333e-1 * t7531 - 0.19025925925925925925e-1 * t7535 + 0.68493333333333333331e-1 * t7538 - 0.34246666666666666665e-1 * t7541 - 0.10274e0 * t7544 + 0.10274e0 * t7547 - 0.17123333333333333333e-1 * t7550;
    (t7689, t7691, t7692, t7694, t7696, t7698, t7710)
}
