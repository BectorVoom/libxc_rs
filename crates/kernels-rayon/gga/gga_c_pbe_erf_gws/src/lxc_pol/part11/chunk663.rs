//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 663/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk663(t329: f64, t369: f64, t838: f64, t2052: f64, t381: f64, t2096: f64, t2454: f64, t4: f64, t959: f64) -> (f64, f64, f64, f64) {
    let t6832 = t329 * t838 * t369;
    let t6854 = 1.0_f64 / t2052 / t381;
    let t6906 = t2454 * t2096;
    let t6967 = t959 * t4;
    (t6832, t6854, t6906, t6967)
}
