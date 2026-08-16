//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1388/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1388(t1256: f64, t6598: f64, t1266: f64, t17344: f64, t17396: f64, t17401: f64, t17721: f64, t17763: f64, t1808: f64, t21267: f64, t21272: f64, t21275: f64, t21283: f64, t21285: f64, t3647: f64, t5270: f64, t5348: f64, t5354: f64, t5386: f64, t5391: f64, t6683: f64) -> f64 {
    let t21287 = t6598 * t1256;
    let t21295 = -0.12862205435420921092e-2_f64 * t17344 * t21267 - 0.48272968547752592738e-2_f64 * t21272 * t1266 + 0.85748036236139473944e-3_f64 * t21275 * t5386 - 0.28582678745379824648e-3_f64 * t3647 * t6683 - 0.28582678745379824648e-3_f64 * t17763 * t1808 + 0.31758531939310916275e-3_f64 * t17721 + 0.14291339372689912324e-3_f64 * t21283 + 0.48272968547752592738e-2_f64 * t21285 - 0.15244095330869239812e-2_f64 * t21287 + 0.30488190661738479624e-2_f64 * t5391 * t5270 - 0.42874018118069736972e-3_f64 * t17401 * t5354 + 0.22866142996303859718e-2_f64 * t17396 * t5348;
    t21295
}
