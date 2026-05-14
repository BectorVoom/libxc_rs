//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 531/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk531<F: Float>(t2204: F, t2215: F, t2209: F, t841: F, t1878: F, t218: F, t344: F, t675: F, t847: F) -> (F, F, F, F, F) {
    let t2216 = t2215 * t2204;
    let t2218 = t841 * t2209;
    let t2221 = t218 * t1878 * t344;
    let t2222 = 0.13692777777777777778e0 * t2221;
    let t2224 = t218 * t675 * t847;
    (t2216, t2218, t2221, t2222, t2224)
}
