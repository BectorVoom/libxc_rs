//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 754/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk754<F: Float>(t362: F, t7298: F, t1431: F, t2352: F, t1422: F, t2300: F, t322: F, t7253: F, t7256: F, t24: F, t2548: F, t1382: F, t7433: F) -> (F, F, F, F, F, F, F) {
    let t10615 = t362 * t7298;
    let t10645 = t1431 * t2352;
    let t10760 = t1422 * t2300;
    let t10825 = t322 * t7253;
    let t10826 = t362 * t7256;
    let t10838 = t24 * t2548;
    let t10856 = t7433 * t1382;
    (t10615, t10645, t10760, t10825, t10826, t10838, t10856)
}
