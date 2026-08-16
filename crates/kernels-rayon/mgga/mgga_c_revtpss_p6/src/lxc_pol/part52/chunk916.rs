//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 916/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk916(t1972: f64, t4857: f64, t1659: f64, t7131: f64, t25515: f64, t4890: f64, t3299: f64, t1028: f64, t1068: f64, t1665: f64, t1675: f64, t25490: f64, t25495: f64, t25529: f64, t25569: f64, t25577: f64, t27471: f64, t4831: f64, t4854: f64, t4896: f64, t7117: f64, t7132: f64) -> (f64, f64) {
    let t27479 = t4857 * t1972;
    let t27489 = t1659 * t7131;
    let t27492 = t25515 * t4890;
    let t27493 = t3299 * t27492;
    let t27496 = -0.28582678745379824648e-3_f64 * t27471 + 0.22866142996303859718e-2_f64 * t25495 * t1665 - 0.42874018118069736972e-3_f64 * t25490 * t1665 - 0.42874018118069736972e-3_f64 * t7117 * t4854 - 0.42874018118069736972e-3_f64 * t27479 * t1028 + 0.28582678745379824648e-3_f64 * t25529 - 0.15244095330869239812e-2_f64 * t25577 * t1675 + 0.28582678745379824648e-3_f64 * t7132 * t4831 + 0.28582678745379824648e-3_f64 * t25569 * t1675 + 0.28582678745379824648e-3_f64 * t27489 * t1068 + 0.85748036236139473944e-3_f64 * t27493 * t4896;
    (t27492, t27496)
}
