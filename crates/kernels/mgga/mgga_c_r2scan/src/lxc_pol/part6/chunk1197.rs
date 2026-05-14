//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1197/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1197<F: Float>(t390: F, t5606: F, t5767: F, t1399: F, t5607: F, t189: F, t5448: F, t5597: F, t1691: F, t1706: F, t1813: F, t1830: F, t1835: F, t21420: F, t21478: F, t22006: F, t22010: F, t22012: F, t22023: F, t22030: F, t22034: F, t22036: F, t226: F, t5572: F, t686: F, t689: F, t705: F) -> (F, F, F, F) {
    let t22039 = 0.12822e1 * t390 * t5767 * t5606;
    let t22041 = 0.85479999999999999998e0 * t1399 * t5607;
    let t22045 = 0.17096e1 * t390 * t5597 * t189 * t5448;
    let t22046 = -0.84214420165938905383e2 * t5572 * t22006 - t22010 + t22012 + 0.19827150884348052633e2 * t686 * t21420 * t689 + 0.23422135608651758058e1 * t1706 * t1830 - 0.69350015718254262349e2 * t1835 * t1691 * t1813 - t22023 - 0.11696447245269292414e1 * t705 * t226 * t21478 - t22030 + t22034 + t22036 + t22039 - t22041 - t22045;
    (t22039, t22041, t22045, t22046)
}
