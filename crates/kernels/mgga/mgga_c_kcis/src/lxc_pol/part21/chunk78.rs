//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 78/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk78<F: Float>(t209: F, t213: F, t208: F, t140: F, t206: F, t155: F, t162: F, rho0: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F) {
    let t214 = t209 * t213;
    let t217 = F::cast_from(1.0_f64) + t208 * t214 / F::cast_from(96.0_f64);
    let t218 = F::ln(t217);
    let t220 = F::cast_from(1.0_f64) + F::cast_from(0.66725e-1_f64) * t218;
    let t221 = F::cast_from(1.0_f64) / t220;
    let t224 = t206 * t221 + F::cast_from(0.69644166666666666665e-2_f64) * t140;
    let t227 = F::cast_from(1.0_f64) + F::cast_from(0.1875e0_f64) * t155 - F::cast_from(0.4046875e-1_f64) * t162;
    let t228 = F::cast_from(1.0_f64) / t227;
    let t232 = F::cast_from(1.0_f64) / rho0;
    let t233 = sigma0 * t232;
    (t214, t217, t220, t221, t224, t227, t228, t232, t233)
}
