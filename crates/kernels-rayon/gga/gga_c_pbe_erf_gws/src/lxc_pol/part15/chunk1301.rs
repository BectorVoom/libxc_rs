//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1301/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1301(t3253: f64, t51255: f64, t14099: f64, t863: f64, t885: f64, t338: f64, t8886: f64, t1125: f64, t51221: f64, t14011: f64, t9393: f64, t14498: f64, t9401: f64) -> (f64, f64, f64, f64, f64) {
    let t54087 = t51255 * t3253;
    let t54088 = 7.0_f64 / 144.0_f64 * t54087;
    let t54090 = t863 * t14099 * t885;
    let t54092 = t54090 * t338 * t8886;
    let t54094 = t1125 * t51221;
    let t54096 = t14011 * t9393;
    let t54098 = t14498 * t9401;
    (t54088, t54092, t54094, t54096, t54098)
}
