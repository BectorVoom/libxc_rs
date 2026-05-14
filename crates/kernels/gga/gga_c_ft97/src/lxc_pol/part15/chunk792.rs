//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 792/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk792<F: Float>(t2440: F, t900: F, t2938: F, t703: F, t70: F, t9651: F, t327: F, t41536: F, t272: F, t41670: F, t41744: F, t2937: F, t326: F, t2404: F, t2680: F, t683: F, t7640: F) -> (F, F, F, F, F, F, F, F, F) {
    let t43122 = t2440 * t900;
    let t43164 = t703 * t2938;
    let t43194 = t70 * t9651;
    let t43195 = t327 * t41536;
    let t43207 = 1.0 / t272 / t41670;
    let t43212 = 0.14978012345679012345e1 * t41744;
    let t43250 = 1.0 / t2937 / t326;
    let t43350 = t2404 * t2680;
    let t43381 = t683 * t7640;
    (t43122, t43164, t43194, t43195, t43207, t43212, t43250, t43350, t43381)
}
