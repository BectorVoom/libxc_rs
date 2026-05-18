//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 755/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk755<F: Float>(t1382: F, t7467: F, t7481: F, t311: F, t7856: F, t10: F, t2595: F, t896: F, t2673: F, t2638: F, t330: F, t1378: F, t530: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10894 = t7467 * t1382;
    let t10917 = t7481 * t1382;
    let t10935 = t311 * t7856;
    let t10959 = t10 * t2595;
    let t10975 = t896 * t1382;
    let t10976 = t10975 * t2673;
    let t10990 = t2638 * t311;
    let t10991 = t330 * t10990;
    let t11007 = t530 * t1378;
    (t10894, t10917, t10935, t10959, t10975, t10976, t10990, t10991, t11007)
}
