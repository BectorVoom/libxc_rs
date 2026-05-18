//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 910/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk910<F: Float>(t40081: F, t7720: F, t7487: F, t8343: F, t8358: F, t8362: F, t2001: F, t2281: F, t326: F, t333: F, t495: F, t515: F, t7230: F, t7231: F, t9109: F) -> (F, F, F, F, F, F) {
    let t40082 = t7720 * t40081;
    let t40084 = t7487 * t8343;
    let t40086 = t7487 * t8358;
    let t40088 = t7487 * t8362;
    let t40092 = t2001 * t326 * t2281 * t333;
    let t40093 = t7720 * t40092;
    let t40098 = t7230 * t7231 * t515 * t9109 * t495;
    (t40082, t40084, t40086, t40088, t40093, t40098)
}
