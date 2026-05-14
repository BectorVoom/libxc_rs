//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 201/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk201<F: Float>(t213: F, t218: F, t211: F, t88: F, t62: F, t215: F, t220: F, t43: F, t238: F, t233: F, t352: F, t354: F, t358: F, t360: F, t241: F, t374: F, t46: F, t379: F, t381: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t214 = t213 <= zeta_threshold;
    let t219 = t218 <= zeta_threshold;
    let t689 = t211 * t88;
    let t690 = t62 - t689;
    let t693 = piecewise3(t214, 0.0, 4.0 / 3.0 * t215 * t690);
    let t694 = -t690;
    let t697 = piecewise3(t219, 0.0, 4.0 / 3.0 * t220 * t694);
    let t699 = (t693 + t697) * t43;
    let t704 = t238 * t238;
    let t705 = 1.0 / t704;
    let t706 = t233 * t705;
    let t711 = -0.1176575e1 * t352 - 0.516475e0 * t354 - 0.2103875e0 * t358 - 0.104195e0 * t360;
    let t712 = 1.0 / t241;
    let t713 = t711 * t712;
    let t719 = t46 * t374;
    let t720 = t379 * t381;
    (t689, t690, t694, t699, t704, t705, t706, t711, t712, t713, t719, t720)
}
