//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1856/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1856(t1028: f64, t1068: f64, t1665: f64, t1675: f64, t25490: f64, t25495: f64, t25529: f64, t25569: f64, t25577: f64, t27471: f64, t27479: f64, t27489: f64, t27493: f64, t4831: f64, t4854: f64, t4896: f64, t7117: f64, t7132: f64) -> f64 {
    let t27496 = -0.28582678745379824648e-3_f64 * t27471 + 0.22866142996303859718e-2_f64 * t25495 * t1665 - 0.42874018118069736972e-3_f64 * t25490 * t1665 - 0.42874018118069736972e-3_f64 * t7117 * t4854 - 0.42874018118069736972e-3_f64 * t27479 * t1028 + 0.28582678745379824648e-3_f64 * t25529 - 0.15244095330869239812e-2_f64 * t25577 * t1675 + 0.28582678745379824648e-3_f64 * t7132 * t4831 + 0.28582678745379824648e-3_f64 * t25569 * t1675 + 0.28582678745379824648e-3_f64 * t27489 * t1068 + 0.85748036236139473944e-3_f64 * t27493 * t4896;
    t27496
}
