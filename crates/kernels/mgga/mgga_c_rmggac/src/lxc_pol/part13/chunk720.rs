//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 720/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk720<F: Float>(t118: F, t1995: F, t2001: F, t498: F, t7335: F, t7341: F, t20: F, t2018: F, t2021: F, t4720: F, t7338: F, t7491: F, t7360: F, t7487: F, t35276: F, t7473: F) -> (F, F, F, F, F, F, F) {
    let t36740 = t2001 * t118 * t1995 * t498;
    let t36748 = t7335 * t7341;
    let t36752 = t4720 * t20 * t2018 * t2021;
    let t36754 = t7335 * t7338;
    let t36756 = t7491 * t7341;
    let t36758 = t7487 * t7360;
    let t36766 = t35276 * t7473;
    (t36740, t36748, t36752, t36754, t36756, t36758, t36766)
}
