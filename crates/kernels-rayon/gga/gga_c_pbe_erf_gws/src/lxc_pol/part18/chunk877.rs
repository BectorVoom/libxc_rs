//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 877/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk877(t367: f64, t6553: f64, t899: f64, t1112: f64, t4394: f64, t3253: f64, t6203: f64, t1154: f64, t6455: f64, t3261: f64, t6416: f64, t3291: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9425 = t899 * t6553 * t367;
    let t9441 = t1112 * t4394;
    let t9447 = 7.0_f64 / 288.0_f64 * t6203 * t3253;
    let t9457 = t6455 * t1154;
    let t9464 = 7.0_f64 / 576.0_f64 * t6416 * t3261;
    let t9474 = 7.0_f64 / 1152.0_f64 * t6416 * t3291;
    (t9425, t9441, t9447, t9457, t9464, t9474)
}
