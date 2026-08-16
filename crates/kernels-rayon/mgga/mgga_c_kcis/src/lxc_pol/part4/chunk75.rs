//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 75/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk75(t206: f64, t209: f64, t208: f64, t140: f64, t155: f64, t162: f64, t166: f64, rho0: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t210 = 0.0_f64 < t206;
    let t212 = piecewise3(t210, t206, -t206);
    let t213 = 1.0_f64 / t212;
    let t214 = t209 * t213;
    let t217 = 1.0_f64 + t208 * t214 / 96.0_f64;
    let t218 = f64::ln(t217);
    let t220 = 1.0_f64 + 0.66725e-1_f64 * t218;
    let t221 = 1.0_f64 / t220;
    let t224 = t206 * t221 + 0.69644166666666666665e-2_f64 * t140;
    let t227 = 1.0_f64 + 0.1875e0_f64 * t155 - 0.4046875e-1_f64 * t162;
    let t228 = 1.0_f64 / t227;
    let t230 = t224 * t228 - t166;
    let t232 = 1.0_f64 / rho0;
    let t233 = sigma0 * t232;
    (t212, t214, t217, t220, t221, t224, t227, t228, t230, t233)
}
