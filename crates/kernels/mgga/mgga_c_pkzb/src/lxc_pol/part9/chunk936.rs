//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 936/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk936<F: Float>(t6827: F, t7195: F, t7205: F, t7215: F, t45: F, t1158: F, t1824: F, t3010: F, t645: F, t2873: F, t5893: F, t730: F) -> (F, F, F, F, F, F) {
    let t7217 = t6827 + t7195 + t7205 + t7215;
    let t7218 = t45 * t7217;
    let t7219 = t1824 * t1158;
    let t7221 = t645 * t3010;
    let t7223 = t2873 * t5893;
    let t7225 = F::cast_from(0.17315859105681463759e2_f64) * t730 * t7223;
    (t7217, t7218, t7219, t7221, t7223, t7225)
}
