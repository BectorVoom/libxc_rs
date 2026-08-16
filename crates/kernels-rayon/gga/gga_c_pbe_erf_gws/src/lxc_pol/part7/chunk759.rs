//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 759/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk759(t2135: f64, t2170: f64, t6220: f64, t2168: f64, t2319: f64, t2339: f64, t1477: f64, t863: f64, t864: f64, t877: f64, t2156: f64, t874: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6222 = t2170 * t2135 * t6220;
    let t6224 = t2168 * t6222 / 16.0_f64;
    let t6225 = t2319 * t2339;
    let t6228 = t863 * t864 * t1477;
    let t6229 = t6228 * t877;
    let t6230 = 35.0_f64 / 144.0_f64 * t6229;
    let t6231 = t2156 * t874;
    (t6222, t6224, t6225, t6228, t6230, t6231)
}
