//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 600/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk600(t1109: f64, t369: f64, t1130: f64, t810: f64, t2494: f64, t339: f64, t2178: f64, t2181: f64, t3028: f64, t340: f64, t870: f64, t871: f64) -> (f64, f64, f64, f64) {
    let t3154 = t1109 * t369;
    let t3159 = t1130 * t810;
    let t3162 = t339 * t2494;
    let t3165 = -t3028 * t339 * t340 + 3.0_f64 * t1130 * t2178 - 12.0_f64 * t2181 * t3159 + 3.0_f64 * t3154 * t871 + 3.0_f64 * t3162 * t870;
    (t3154, t3159, t3162, t3165)
}
