//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 751/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk751<F: Float>(t1095: F, t5873: F, t1083: F, t5804: F, t1979: F, t2848: F, t1107: F, t5493: F, t1100: F, t1976: F, t1088: F, t1937: F, t1079: F, t1878: F, t218: F) -> (F, F, F, F, F, F, F) {
    let t7247 = t1095 * t5873;
    let t7285 = t1083 * t5804;
    let t7299 = t2848 * t1979;
    let t7308 = t1107 * t5493;
    let t7315 = t1100 * t1976;
    let t7324 = t1088 * t1937;
    let t7332 = t218 * t1878 * t1079;
    (t7247, t7285, t7299, t7308, t7315, t7324, t7332)
}
