//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1269/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1269<F: Float>(t44: F, t23834: F, t41: F, t4878: F, t898: F, t1213: F, t1216: F, t1219: F, t415: F, t40: F, t2469: F, t409: F, t1361: F, t18794: F, t2466: F, t35: F, t48: F, t4905: F, t4913: F, t4938: F, t6976: F, t6979: F, t889: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t45 = t44 <= zeta_threshold;
    let t23835 = 3.0 * t23834;
    let t23837 = t41 * t898 * t4878;
    let t23842 = t1216 * t1213;
    let t23845 = t415 * t1219;
    let t23851 = t40 * t415;
    let t23854 = t1216 * t1219;
    let t23862 = 32.0 * t2469 * t409;
    let t23864 = piecewise3(t45, 0.0, 40.0 / 81.0 * t18794 * t889 * t4905 - 16.0 / 9.0 * t4938 * t35 * t23842 - 8.0 / 9.0 * t6976 * t23845 + 8.0 / 3.0 * t1361 * t1216 * t415 - 8.0 * t6979 * t23851 + 8.0 / 3.0 * t6979 * t23854 + 4.0 / 9.0 * t2466 * t4913 - 16.0 * t48 * t40 + t23862);
    (t23835, t23837, t23842, t23845, t23851, t23854, t23864)
}
