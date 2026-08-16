//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1378/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1378(t19345: f64, t5790: f64, t18350: f64, t20275: f64, t5492: f64, t18338: f64, t18360: f64, t18366: f64, t18666: f64, t19342: f64, t19349: f64, t20246: f64, t20264: f64, t20282: f64, t62007: f64, t62277: f64, t62356: f64, t6304: f64, t65152: f64, t65162: f64, t65202: f64) -> f64 {
    let t67472 = t5790 * t19345;
    let t67474 = 160.0_f64 / 9.0_f64 * t18350 * t67472;
    let t67480 = 32.0_f64 / 9.0_f64 * t5492 * t20275;
    let t67489 = 20.0_f64 * t62277 * t19342 + 20.0_f64 * t18666 * t65202 + 20.0_f64 * t18666 * t65162 + 10.0_f64 * t18666 * t65152 - t67474 + 20.0_f64 / 3.0_f64 * t62007 * t20264 + 10.0_f64 / 3.0_f64 * t19349 * t62356 + t67480 - 4.0_f64 / 3.0_f64 * t18338 * t6304 - 2.0_f64 / 3.0_f64 * t18366 * t6304 - 4.0_f64 / 3.0_f64 * t5492 * t20282 - 5.0_f64 / 3.0_f64 * t20246 * t18360;
    t67489
}
