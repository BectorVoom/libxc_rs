//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1124/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1124<F: Float>(t4422: F, t828: F, t2123: F, t2120: F, t6203: F, t6208: F, t6563: F, t6711: F, t2074: F, t816: F, t2271: F, t6277: F) -> (F, F, F, F, F, F) {
    let t20189 = t4422 * t828;
    let t20190 = t20189 * t2123;
    let t20191 = t2120 * t20190;
    let t20192 = F::cast_from(35.0_f64) / F::cast_from(72.0_f64) * t20191;
    let t20193 = t6203 * t6208;
    let t20196 = F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t6711 * t6563;
    let t20197 = t816 * t2074;
    let t20202 = t2271 * t6277;
    (t20189, t20192, t20193, t20196, t20197, t20202)
}
