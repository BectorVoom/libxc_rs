//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 735/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk735<F: Float>(t471: F, t4933: F, t97: F, t4696: F, t4703: F, t4880: F, t4882: F, t4884: F, t4887: F, t4891: F, t4893: F, t4895: F, t4897: F, t4899: F, t4901: F, t139: F) -> (F, F, F) {
    let t4935 = t97 * t471 * t4933;
    let t4936 = 3.0 * t4935;
    let t4937 = t4696 + t4880 + t4882 + t4884 + t4887 - t4891 + t4893 - t4895 + t4703 - t4897 + t4899 + t4901;
    let t4938 = 1.0 / t139;
    (t4936, t4937, t4938)
}
