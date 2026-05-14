//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1266/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1266<F: Float>(t20747: F, t247: F, t3719: F, t369: F, t6593: F, t475: F, t467: F, t1260: F, t17307: F, t1256: F, t6602: F, t6595: F, t6598: F, t1266: F, t17344: F, t17396: F, t17401: F, t17721: F, t17763: F, t1808: F, t3647: F, t5270: F, t5348: F, t5354: F, t5386: F, t5391: F, t6683: F) -> (F,) {
    let t21267 = t247 * t3719 * t20747;
    let t21270 = t6593 * t369;
    let t21271 = t475 * t21270;
    let t21272 = t467 * t21271;
    let t21275 = t17307 * t1260;
    let t21283 = t6602 * t1256;
    let t21285 = t6595 * t1256;
    let t21287 = t6598 * t1256;
    let t21295 = -0.12862205435420921092e-2 * t17344 * t21267 - 0.48272968547752592738e-2 * t21272 * t1266 + 0.85748036236139473944e-3 * t21275 * t5386 - 0.28582678745379824648e-3 * t3647 * t6683 - 0.28582678745379824648e-3 * t17763 * t1808 + 0.31758531939310916275e-3 * t17721 + 0.14291339372689912324e-3 * t21283 + 0.48272968547752592738e-2 * t21285 - 0.15244095330869239812e-2 * t21287 + 0.30488190661738479624e-2 * t5391 * t5270 - 0.42874018118069736972e-3 * t17401 * t5354 + 0.22866142996303859718e-2 * t17396 * t5348;
    (t21295,)
}
