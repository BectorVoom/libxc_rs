//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 936/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk936<F: Float>(t10426: F, t22267: F, t5182: F, t4817: F, t8940: F, t1869: F, t4581: F, t8954: F, t1799: F, t1757: F, t8672: F, t1899: F, t5062: F, t2527: F, t7069: F, t1801: F) -> (F, F, F, F, F, F, F) {
    let t22268 = t10426 * t22267;
    let t22269 = t5182 * t22268;
    let t22271 = t4817 * t8940;
    let t22272 = t1869 * t22271;
    let t22274 = t4581 * t8954;
    let t22275 = t1799 * t22274;
    let t22278 = t8672 * t1757;
    let t22279 = t1899 * t22278;
    let t22280 = t5062 * t22279;
    let t22281 = t1869 * t22280;
    let t22283 = t2527 * t7069;
    let t22284 = t1801 * t22283;
    (t22269, t22272, t22275, t22278, t22281, t22283, t22284)
}
