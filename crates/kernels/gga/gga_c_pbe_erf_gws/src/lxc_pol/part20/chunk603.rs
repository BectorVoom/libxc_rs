//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 603/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk603<F: Float>(t3178: F, t337: F, t2147: F, t2146: F, t2170: F, t875: F, t2168: F, t1140: F, t2206: F, t1105: F, t810: F) -> (F, F, F, F, F, F, F, F) {
    let t3179 = t337 * t3178;
    let t3180 = t2147 * t3179;
    let t3182 = t2146 * t3180 / F::cast_from(48.0_f64);
    let t3184 = t2170 * t3178 * t875;
    let t3186 = t2168 * t3184 / F::cast_from(48.0_f64);
    let t3187 = t2206 * t1140;
    let t3188 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t3187;
    let t3189 = t1105 * t810;
    (t3179, t3180, t3182, t3184, t3186, t3187, t3188, t3189)
}
