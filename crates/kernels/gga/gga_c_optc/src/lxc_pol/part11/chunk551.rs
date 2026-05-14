//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 551/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk551<F: Float>(t4579: F, t85: F, t3387: F, t3389: F, t3391: F, t185: F, t4595: F, t108: F, t176: F) -> (F, F, F, F, F) {
    let t4604 = 0.19751789702565206229e-1 * t4579 * t85;
    let t4606 = 0.11696446794910408142e1 * t3387;
    let t4607 = 8.0 * t3389;
    let t4608 = 8.0 * t3391;
    let t4609 = t185 * t4595;
    let t4611 = t176 * t4609 * t108;
    (t4604, t4606, t4607, t4608, t4611)
}
