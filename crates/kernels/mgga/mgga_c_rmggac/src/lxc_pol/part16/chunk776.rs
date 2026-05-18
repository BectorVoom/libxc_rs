//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 776/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk776<F: Float>(t7335: F, t7338: F, t7341: F, t7491: F, t1969: F, t34846: F, t7345: F, t7927: F, t35207: F, t7354: F, t2019: F, t2165: F, t7926: F) -> (F, F, F, F, F, F) {
    let t36754 = t7335 * t7338;
    let t36756 = t7491 * t7341;
    let t36772 = t34846 * t1969;
    let t36796 = t7345 * t7927;
    let t36801 = t35207 * t7354;
    let t36804 = t2019 * t7926 * t2165;
    (t36754, t36756, t36772, t36796, t36801, t36804)
}
