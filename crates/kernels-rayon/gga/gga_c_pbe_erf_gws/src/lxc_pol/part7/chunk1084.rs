//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1084/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1084(t2423: f64, t804: f64, t1332: f64, t296: f64, t6073: f64, t6072: f64, t6074: f64, t793: f64, t18471: f64, t18474: f64, t18477: f64, t18479: f64, t18512: f64, t18514: f64, t18518: f64, t18521: f64, t18523: f64, t18527: f64, t18529: f64, t2074: f64, t2424: f64, t6838: f64, t810: f64, t8556: f64) -> (f64, f64, f64) {
    let t19477 = t804 * t2423;
    let t19482 = 0.47400060215270560269e1_f64 * t6073 * t1332 * t296;
    let t19487 = t793 * t6072 * t6074;
    let t19488 = 0.18960024086108224108e1_f64 * t19487;
    let t19492 = 18.0_f64 * t2074 * t2424 * t804 + 12.0_f64 * t6838 * t804 * t810 - 36.0_f64 * t19477 * t8556 - t18471 - t18474 + t18477 + t18479 + t18512 - t18514 + t18518 + t18521 - t18523 + t18527 + t18529 + t19482 - t19488;
    (t19482, t19488, t19492)
}
