//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1057/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1057<F: Float>(t2171: F, t2345: F, t9375: F, t2289: F, t3283: F, t2300: F, t8804: F, t904: F, t3242: F, t6627: F, t2343: F, t6592: F, t6597: F, t9124: F, t9129: F, t9133: F, t9137: F, t9138: F, t9140: F, t929: F) -> (F, F, F) {
    let t9588 = t2345 * t9375 * t2171;
    let t9592 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t2289 * t3283;
    let t9594 = t2300 * t904 * t8804;
    let t9598 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t6627 * t3242;
    let t9599 = -t6592 - t6597 - t9124 + t9129 + t2343 * t9588 / F::cast_from(192.0_f64) + t9133 + t9137 + t9592 + F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t929 * t9594 + t9138 - t9598 + t9140;
    (t9588, t9594, t9599)
}
