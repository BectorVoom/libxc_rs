//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 199/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk199<F: Float>(t213: F, t218: F, t211: F, t88: F, t62: F, t215: F, t220: F, t43: F, t238: F, t233: F, t352: F, t354: F, t358: F, t360: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t214 = t213 <= zeta_threshold;
    let t219 = t218 <= zeta_threshold;
    let t689 = t211 * t88;
    let t690 = t62 - t689;
    let t693 = piecewise3::<F>(t214, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t215 * t690);
    let t694 = -t690;
    let t697 = piecewise3::<F>(t219, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t220 * t694);
    let t699 = (t693 + t697) * t43;
    let t704 = t238 * t238;
    let t705 = F::cast_from(1.0_f64) / t704;
    let t706 = t233 * t705;
    let t711 = -F::cast_from(0.1176575e1_f64) * t352 - F::cast_from(0.516475e0_f64) * t354 - F::cast_from(0.2103875e0_f64) * t358 - F::cast_from(0.104195e0_f64) * t360;
    (t690, t694, t699, t704, t705, t706, t711)
}
