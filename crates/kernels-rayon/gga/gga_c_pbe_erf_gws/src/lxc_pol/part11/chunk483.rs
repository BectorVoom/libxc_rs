//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 483/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk483(t1136: f64, t2164: f64, t1109: f64, t369: f64, t1105: f64, t5: f64, t337: f64, t2147: f64) -> (f64, f64, f64, f64) {
    let t3145 = t2164 * t1136;
    let t3154 = t1109 * t369;
    let t3178 = t5 * t1105;
    let t3179 = t337 * t3178;
    let t3180 = t2147 * t3179;
    (t3145, t3154, t3179, t3180)
}
