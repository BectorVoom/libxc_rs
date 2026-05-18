//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 199/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk199<F: Float>(t211: F, t88: F, t238: F, t233: F, t352: F, t354: F, t358: F, t360: F, t241: F, t374: F, t46: F, t379: F, t381: F) -> (F, F, F, F, F, F, F, F, F) {
    let t689 = t211 * t88;
    let t704 = t238 * t238;
    let t705 = F::new(1.0) / t704;
    let t706 = t233 * t705;
    let t711 = -F::new(0.1176575e1) * t352 - F::new(0.516475e0) * t354 - F::new(0.2103875e0) * t358 - F::new(0.104195e0) * t360;
    let t712 = F::new(1.0) / t241;
    let t713 = t711 * t712;
    let t719 = t46 * t374;
    let t720 = t379 * t381;
    (t689, t704, t705, t706, t711, t712, t713, t719, t720)
}
