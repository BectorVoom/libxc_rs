//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1044/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1044<F: Float>(t2300: F, t8759: F, t904: F, t2253: F, t2277: F, t8925: F, t8927: F, t8930: F, t8932: F, t8936: F, t8938: F, t914: F, t929: F, t9434: F, t9438: F, t9443: F, t9447: F, t9449: F) -> (F, F) {
    let t9453 = t2300 * t904 * t8759;
    let t9456 = -t8925 - t2253 * t9434 / F::new(384.0) - t8927 - t8930 + t8932 - t2277 * t9438 / F::new(768.0) + t2277 * t9443 / F::new(768.0) + t8936 - t8938 + t9447 - t914 * t9449 / F::new(1536.0) + F::new(5.0) / F::new(768.0) * t929 * t9453;
    (t9453, t9456)
}
