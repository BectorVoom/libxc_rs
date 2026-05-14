//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 305/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk305<F: Float>(t322: F, t327: F, t829: F, t834: F, t330: F, t828: F) -> (F, F, F) {
    let t332 = 0.25e1 < t322;
    let t837 = -0.64e0 * t829 * t327 - 0.64e0 * t834 * t829;
    let t838 = t837 * t330;
    let t839 = piecewise3(t332, 0.0, t828);
    (t837, t838, t839)
}
