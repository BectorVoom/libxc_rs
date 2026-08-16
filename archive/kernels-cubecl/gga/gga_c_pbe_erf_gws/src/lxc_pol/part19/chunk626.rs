//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 626/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk626<F: Float>(t43: F, t1402: F, t3342: F, t3346: F, t47: F, t954: F, zeta_threshold: F) -> (F, F) {
    let t44 = t43 <= zeta_threshold;
    let t3350 = piecewise3::<F>(t44, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1402 * t3342 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t47 * t3346);
    let t3351 = t954 * t954;
    (t3350, t3351)
}
