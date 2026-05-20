//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1386/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1386<F: Float>(t1256: F, t6598: F, t1266: F, t17344: F, t17396: F, t17401: F, t17721: F, t17763: F, t1808: F, t21267: F, t21272: F, t21275: F, t21283: F, t21285: F, t3647: F, t5270: F, t5348: F, t5354: F, t5386: F, t5391: F, t6683: F) -> F {
    let t21287 = t6598 * t1256;
    let t21295 = -F::cast_from(0.12862205435420921092e-2_f64) * t17344 * t21267 - F::cast_from(0.48272968547752592738e-2_f64) * t21272 * t1266 + F::cast_from(0.85748036236139473944e-3_f64) * t21275 * t5386 - F::cast_from(0.28582678745379824648e-3_f64) * t3647 * t6683 - F::cast_from(0.28582678745379824648e-3_f64) * t17763 * t1808 + F::cast_from(0.31758531939310916275e-3_f64) * t17721 + F::cast_from(0.14291339372689912324e-3_f64) * t21283 + F::cast_from(0.48272968547752592738e-2_f64) * t21285 - F::cast_from(0.15244095330869239812e-2_f64) * t21287 + F::cast_from(0.30488190661738479624e-2_f64) * t5391 * t5270 - F::cast_from(0.42874018118069736972e-3_f64) * t17401 * t5354 + F::cast_from(0.22866142996303859718e-2_f64) * t17396 * t5348;
    t21295
}
