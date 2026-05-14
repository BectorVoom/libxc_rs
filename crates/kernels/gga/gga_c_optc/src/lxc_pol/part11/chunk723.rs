//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 723/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk723<F: Float>(t1422: F, t2300: F, t322: F, t7253: F, t362: F, t7256: F, t24: F, t2548: F, t1382: F, t7433: F, t7467: F, t7481: F, t311: F, t7856: F, t10: F, t2595: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10760 = t1422 * t2300;
    let t10825 = t322 * t7253;
    let t10826 = t362 * t7256;
    let t10838 = t24 * t2548;
    let t10856 = t7433 * t1382;
    let t10894 = t7467 * t1382;
    let t10917 = t7481 * t1382;
    let t10935 = t311 * t7856;
    let t10959 = t10 * t2595;
    (t10760, t10825, t10826, t10838, t10856, t10894, t10917, t10935, t10959)
}
