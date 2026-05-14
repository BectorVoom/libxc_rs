//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 737/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk737<F: Float>(t4959: F, t88: F, t41: F, t1524: F, t378: F, t735: F) -> (F, F, F, F, F) {
    let t4960 = t4959 * t88;
    let t4961 = t41 * t4960;
    let t4962 = t378 * t1524;
    let t4963 = t735 * t4962;
    let t4964 = 0.32530743900905219526e-1 * t4963;
    (t4960, t4961, t4962, t4963, t4964)
}
