//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 898/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk898<F: Float>(t1366: F, t3315: F, t3322: F, t27: F, t3027: F, t545: F, t1377: F, t1403: F, t97: F, t1410: F, t1767: F, t184: F, t186: F, t30: F, t32: F) -> (F, F, F, F, F, F) {
    let t10743 = t3315 * t1366;
    let t10746 = F::new(0.4328416544945937) * t3322 * t1366;
    let t10757 = t3027 * t27 * t545;
    let t10760 = t1403 * t97 * t1377;
    let t10764 = F::new(0.06709045644666203) * t1410 * t97 * t1377;
    let t10769 = F::new(2.8503734567901235e-05) * t184 * t1767 * t30 * t32 * t186;
    (t10743, t10746, t10757, t10760, t10764, t10769)
}
