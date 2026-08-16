//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1957/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1957(t1227: f64, t1238: f64, t1266: f64, t26867: f64, t26870: f64, t26877: f64, t29083: f64, t29086: f64, t29089: f64, t29097: f64, t29100: f64, t5335: f64, t5343: f64, t5348: f64, t5354: f64, t5369: f64, t5397: f64, t5402: f64, t7607: f64, t7624: f64) -> f64 {
    let t29107 = -0.28582678745379824648e-3_f64 * t7624 * t5397 + 0.15244095330869239812e-2_f64 * t29083 * t1266 - 0.42874018118069736972e-3_f64 * t29086 * t1238 + t29089 * t1227 / 108.0_f64 - t7607 * t5369 / 288.0_f64 - t26877 - 0.28582678745379824648e-3_f64 * t26867 * t5402 + 0.85748036236139473944e-3_f64 * t29097 * t5343 - 0.42874018118069736972e-3_f64 * t29100 * t5335 - 0.42874018118069736972e-3_f64 * t26870 * t5348 - 0.42874018118069736972e-3_f64 * t26870 * t5354;
    t29107
}
