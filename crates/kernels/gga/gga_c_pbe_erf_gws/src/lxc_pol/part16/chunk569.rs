//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 569/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk569<F: Float>(t43: F, t318: F, t1428: F, t1098: F, t19: F, t796: F, t801: F, t1402: F, t950: F, t34: F, t47: F, t418: F, t532: F, param_gamma: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t2429 = param_gamma * t318;
    let t2449 = F::cast_from(4.0_f64) * t1428;
    let t2454 = t1098 * t796 * t19;
    let t2455 = t2454 * t801;
    let t2456 = F::cast_from(0.41076328840066666668e0_f64) * t2455;
    let t2457 = t1402 * t950;
    let t2460 = t47 * t34;
    let t2464 = piecewise3::<F>(t44, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t2457 * t418 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t2460 * t532);
    (t2429, t2449, t2454, t2456, t2457, t2460, t2464)
}
