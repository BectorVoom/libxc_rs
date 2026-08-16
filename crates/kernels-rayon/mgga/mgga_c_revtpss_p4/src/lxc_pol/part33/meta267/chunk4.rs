//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1193/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1193(t1227: f64, t1238: f64, t1252: f64, t1266: f64, t484: f64, t7606: f64, t7607: f64, t7610: f64, t7613: f64, t7618: f64, t7622: f64, t7624: f64) -> f64 {
    let t7627 = t7606 - t7607 * t1227 / 288.0_f64 + 0.42874018118069736972e-3_f64 * t7610 * t484 - 0.42874018118069736972e-3_f64 * t7613 * t1238 + 0.42874018118069736972e-3_f64 * t7618 * t1252 + t7622 - 0.28582678745379824648e-3_f64 * t7624 * t1266;
    t7627
}
