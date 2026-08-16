//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1237/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1237(t467: f64, t8184: f64, t1782: f64, t1791: f64, t1797: f64, t1808: f64, t464: f64, t484: f64, t7606: f64, t7607: f64, t7613: f64, t7618: f64, t7622: f64, t7624: f64, t8172: f64, t8177: f64) -> (f64, f64) {
    let t8185 = t467 * t8184;
    let t8190 = -t8172 * t464 / 36.0_f64 + t7606 - t7607 * t1782 / 288.0_f64 + 0.42874018118069736972e-3_f64 * t8177 * t484 - 0.42874018118069736972e-3_f64 * t7613 * t1791 + 0.42874018118069736972e-3_f64 * t7618 * t1797 - 0.22866142996303859718e-2_f64 * t8185 * t484 + t7622 - 0.28582678745379824648e-3_f64 * t7624 * t1808;
    (t8185, t8190)
}
