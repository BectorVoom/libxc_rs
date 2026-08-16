//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1150/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1150(t12779: f64, t2615: f64, t42014: f64, t42037: f64, t33149: f64, t33152: f64, t42050: f64, t25349: f64, t48291: f64, t48295: f64, t48299: f64, t48303: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t48305 = 32.0_f64 / 15.0_f64 * t2615 * t12779;
    let t48306 = 32.0_f64 / 15.0_f64 * t42014;
    let t48307 = 32.0_f64 / 45.0_f64 * t42037;
    let t48309 = 64.0_f64 / 135.0_f64 * t33149;
    let t48310 = 32.0_f64 / 135.0_f64 * t33152;
    let t48311 = 32.0_f64 / 45.0_f64 * t42050;
    let t48312 = -t48291 - t48295 + t48299 + t48303 + t48305 + t48306 + t48307 + 32.0_f64 / 81.0_f64 * t25349 - t48309 + t48310 - t48311;
    (t48305, t48306, t48307, t48309, t48310, t48311, t48312)
}
