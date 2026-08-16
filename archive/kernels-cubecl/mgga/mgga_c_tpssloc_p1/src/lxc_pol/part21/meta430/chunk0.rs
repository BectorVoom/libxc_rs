//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1962/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1962<F: Float>(t15293: F, t3449: F, t11529: F, t1709: F, t1174: F, t1714: F, t3475: F, t460: F, t4934: F, t3432: F, t4889: F, t3450: F, t3966: F) -> (F, F, F, F, F, F, F) {
    let t15294 = t3449 * t15293;
    let t15299 = t11529 * t1709;
    let t15300 = t1174 * t15299;
    let t15303 = t1714 * t3475 * t460;
    let t15304 = t4934 * t15303;
    let t15307 = t4889 * t3432;
    let t15313 = t3450 * t3966;
    (t15294, t15299, t15300, t15303, t15304, t15307, t15313)
}
