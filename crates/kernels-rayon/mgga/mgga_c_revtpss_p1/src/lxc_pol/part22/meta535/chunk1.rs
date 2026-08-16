//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2338/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2338(t1222: f64, t1266: f64, t12853: f64, t17401: f64, t17405: f64, t17412: f64, t17417: f64, t17420: f64, t17425: f64, t17426: f64, t17429: f64, t3689: f64, t3694: f64, t3723: f64, t5335: f64, t5340: f64, t5343: f64, t5373: f64) -> f64 {
    let t17432 = -0.42874018118069736972e-3_f64 * t17401 * t3723 - t1222 * t17405 / 288.0_f64 + t5373 * t3689 / 108.0_f64 + t5373 * t3694 / 54.0_f64 + 0.15244095330869239812e-2_f64 * t17412 * t1266 + 0.31758531939310916276e-4_f64 * t17417 + 0.85748036236139473944e-3_f64 * t5340 * t17420 + t12853 + t17425 + 0.85748036236139473944e-3_f64 * t17426 * t5343 - 0.42874018118069736972e-3_f64 * t17429 * t5335;
    t17432
}
