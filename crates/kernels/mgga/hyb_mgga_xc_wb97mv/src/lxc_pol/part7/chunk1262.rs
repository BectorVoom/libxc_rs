//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1262/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1262<F: Float>(t11025: F, t238: F, t800: F, t11029: F, t2187: F, t242: F, t4153: F, t10976: F, t778: F, t226: F, t30776: F, t22475: F, t22478: F, t22481: F, t22498: F, t22501: F, t22620: F, t22633: F) -> (F, F, F, F, F, F) {
    let t30918 = t238 * t800 * t11025;
    let t30921 = t238 * t800 * t11029;
    let t30925 = t238 * t242 * t2187 * t4153;
    let t30929 = t238 * t242 * t778 * t10976;
    let t30933 = t238 * t242 * t226 * t30776;
    let t30940 = -0.32862666666666666666e0 * t30918 - 0.32862666666666666666e0 * t30921 + 0.24647e0 * t30925 + 0.49294e0 * t30929 + 0.24647e0 * t30933 - 0.18602370370370370371e1 * t22498 + 0.39862222222222222223e0 * t22501 + t22633 + t22620 + 0.27385555555555555556e0 * t22478 - 0.1460562962962962963e1 * t22475 + 0.27385555555555555556e0 * t22481;
    (t30918, t30921, t30925, t30929, t30933, t30940)
}
