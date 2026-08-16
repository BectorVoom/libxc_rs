//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1158/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1158<F: Float>(t1238: F, t8245: F, t179: F, t3730: F, t404: F, t6380: F, t8397: F, t2395: F, t3876: F, t5939: F, t6404: F, t5728: F, t919: F) -> (F, F, F, F, F, F) {
    let t28113 = t1238 * t8245;
    let t28121 = t404 * t179 * t6380 * t3730;
    let t28123 = t1238 * t8397;
    let t28128 = t2395 * t5939 * t3876;
    let t28138 = t6404 * t3730;
    let t28147 = t5728 * t919;
    (t28113, t28121, t28123, t28128, t28138, t28147)
}
