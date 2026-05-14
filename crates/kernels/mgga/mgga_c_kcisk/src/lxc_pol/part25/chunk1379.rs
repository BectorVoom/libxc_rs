//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1379/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1379<F: Float>(t10013: F, t12261: F, t2804: F, t118069: F, t34456: F, t9724: F, t118051: F, t33196: F, t34422: F, t5014: F, t10000: F, t113181: F, t116970: F, t116973: F, t116976: F, t118134: F, t18340: F, t33180: F, t34444: F, t34457: F, t9728: F, t9748: F) -> (F,) {
    let t118316 = t2804 * t12261 * t10013;
    let t118324 = t2804 * t118069;
    let t118326 = t9724 * t34456;
    let t118330 = 0.13402777777777777778e-2 * t33196 * t118051;
    let t118334 = t5014 * t34422;
    let t118340 = -0.60312500000000000001e-2 * t34444 * t33180 - 0.11574074074074074074e-2 * t118316 + 0.10416666666666666667e-1 * t34457 * t9748 - 0.10416666666666666667e-1 * t10000 * t33180 + 0.10416666666666666667e-1 * t34457 * t9728 - 0.11574074074074074074e-2 * t118324 + 0.40208333333333333334e-2 * t118326 * t9728 - t118330 - 0.61905925925925925924e-2 * t116970 + 0.11607361111111111111e-2 * t116973 - 0.17411041666666666666e-2 * t116976 - 0.69444444444444444445e-2 * t113181 * t118334 * t18340 - 0.34722222222222222222e-2 * t113181 * t118134;
    (t118340,)
}
