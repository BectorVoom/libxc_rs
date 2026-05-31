//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1051/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1051<F: Float>(t3252: F, t9521: F, t1123: F, t2255: F, t6211: F, t2312: F, t6275: F, t6403: F, t6579: F, t6637: F, t8980: F, t8985: F, t8993: F, t8998: F, t9002: F, t9007: F, t9506: F, t9509: F, t9512: F, t9517: F) -> (F, F, F) {
    let t9522 = t3252 * t9521;
    let t9527 = t2255 * t1123 * t6211;
    let t9530 = t8980 + t8985 + t8993 + t6275 * t9506 / F::cast_from(96.0_f64) + t6637 * t9509 / F::cast_from(384.0_f64) - t8998 + F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t6579 * t9512 - t9002 + t9007 - t2312 * t9517 / F::cast_from(96.0_f64) + t2312 * t9522 / F::cast_from(384.0_f64) + F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t6403 - t2312 * t9527 / F::cast_from(192.0_f64);
    (t9522, t9527, t9530)
}
