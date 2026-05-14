//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 564/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk564<F: Float>(t3102: F, t871: F, t1196: F, t2281: F, t870: F, t2175: F, t2285: F, t3017: F, t3028: F) -> (F, F, F, F) {
    let t3103 = t3102 * t871;
    let t3106 = t1196 * t2281;
    let t3107 = t3106 * t870;
    let t3113 = t2285 - 0.92708333333333333333e-2 * t2175 - 0.92708333333333333333e-2 * t3017 + 0.278125e-1 * t3028;
    (t3103, t3106, t3107, t3113)
}
