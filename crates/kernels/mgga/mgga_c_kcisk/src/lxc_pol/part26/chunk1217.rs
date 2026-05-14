//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1217/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1217<F: Float>(t382: F, t8072: F, t3929: F, t7828: F, t1219: F, t25906: F, t7906: F, t1322: F, t8054: F, t1591: F, t8398: F, t8335: F, t2326: F, t6581: F, t4346: F, t8396: F) -> (F, F, F, F, F, F, F, F, F) {
    let t80050 = t382 * t8072;
    let t80222 = t7828 * t3929;
    let t80804 = t25906 * t1219;
    let t80875 = t382 * t7906;
    let t81032 = t8054 * t1322;
    let t83235 = t8398 * t1591;
    let t83433 = t8335 * t1591;
    let t83438 = t2326 * t6581;
    let t84689 = t8396 * t4346;
    (t80050, t80222, t80804, t80875, t81032, t83235, t83433, t83438, t84689)
}
