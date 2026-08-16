//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 932/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk932(t19775: f64, t824: f64, t2169: f64, t2200: f64, t329: f64, t2079: f64, t19561: f64, t6094: f64, t825: f64, t2365: f64, t6472: f64, t2409: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19905 = t824 * t19775;
    let t20091 = t329 * t2200 * t2169;
    let t20133 = t2079 * t2079;
    let t20137 = t19561 * t6094;
    let t20138 = t20137 * t825;
    let t20142 = t6472 * t2365;
    let t20154 = t2169 * t2409;
    (t19905, t20091, t20133, t20137, t20138, t20142, t20154)
}
