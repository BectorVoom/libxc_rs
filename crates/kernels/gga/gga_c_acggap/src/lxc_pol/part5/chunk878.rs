//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 878/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk878<F: Float>(t1308: F, t3883: F, t4119: F, t857: F, t12200: F, t557: F, t4131: F, t880: F, t3645: F, t547: F, t1603: F, t862: F, t865: F, t1659: F, t3892: F, t3035: F, t3923: F, t545: F) -> (F, F, F, F, F, F, F, F) {
    let t15238 = t1308 * t3883;
    let t15247 = t857 * t4119;
    let t15249 = t12200 * t557;
    let t15251 = t4131 * t880;
    let t15253 = t3645 * t547;
    let t15259 = t862 * t1603 * t865;
    let t15262 = t3892 * t1659;
    let t15265 = t3035 * t545 * t3923;
    (t15238, t15247, t15249, t15251, t15253, t15259, t15262, t15265)
}
