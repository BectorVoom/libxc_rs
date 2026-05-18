//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 830/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk830<F: Float>(t43: F, t1403: F, t1407: F, t2457: F, t2460: F, t39: F, t47: F, t532: F, t6933: F, t6936: F, t6937: F, t4767: F, t954: F, zeta_threshold: F) -> (F, F) {
    let t44 = t43 <= zeta_threshold;
    let t6947 = piecewise3::<f64>(t44, F::new(0.0), -F::new(8.0) / F::new(27.0) * t6933 * t1403 + F::new(16.0) / F::new(9.0) * t6936 * t6937 + F::new(4.0) / F::new(9.0) * t2457 * t1407 + F::new(8.0) / F::new(3.0) * t47 * t532 - F::new(8.0) * t2460 * t39);
    let t6948 = t4767 * t954;
    (t6947, t6948)
}
