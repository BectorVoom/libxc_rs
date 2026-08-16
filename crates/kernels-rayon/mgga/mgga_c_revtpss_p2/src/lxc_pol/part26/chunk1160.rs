//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1160/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1160(t25162: f64, t95296: f64, t2047: f64, t92576: f64, t92584: f64, t2247: f64, t2251: f64, t68: f64, t26182: f64, t6960: f64, t92565: f64, t92588: f64, t95284: f64, t95286: f64, t95288: f64, t95290: f64, t95294: f64) -> f64 {
    let t95297 = t25162 * t95296;
    let t95303 = t2047 * t92576;
    let t95306 = t2047 * t92584;
    let t95310 = t2247 * t2251 * t68;
    let t95313 = 80.0_f64 / 3.0_f64 * t95284 + 40.0_f64 / 3.0_f64 * t95286 + 32.0_f64 / 3.0_f64 * t95288 + 16.0_f64 / 3.0_f64 * t95290 - 440.0_f64 / 9.0_f64 * t95294 - 160.0_f64 / 3.0_f64 * t95297 + 20.0_f64 * t92565 * t26182 + 10.0_f64 * t92588 * t26182 + 20.0_f64 * t25162 * t95303 + 10.0_f64 * t25162 * t95306 + 10.0_f64 * t95310 * t6960;
    t95313
}
