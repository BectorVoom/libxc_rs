//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1294/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1294<F: Float>(t44: F, t2512: F, t409: F, t1212: F, t1216: F, t19347: F, t23842: F, t23845: F, t23851: F, t23854: F, t2509: F, t35: F, t40: F, t415: F, t472: F, t4904: F, t4905: F, t4913: F, t7059: F, t7062: F, t889: F, zeta_threshold: F) -> (F,) {
    let t45 = t44 <= zeta_threshold;
    let t24382 = 16.0 * t2512 * t409;
    let t24384 = piecewise3(t45, 0.0, -56.0 / 81.0 * t19347 * t889 * t4905 + 16.0 / 9.0 * t4904 * t35 * t23842 + 8.0 / 9.0 * t7059 * t23845 - 4.0 / 3.0 * t1212 * t1216 * t415 + 4.0 * t7062 * t23851 - 4.0 / 3.0 * t7062 * t23854 - 2.0 / 9.0 * t2509 * t4913 - 8.0 * t472 * t40 + t24382);
    (t24384,)
}
