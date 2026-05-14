//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 724/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk724<F: Float>(t2411: F, t678: F, t7920: F, t1986: F, t326: F, t495: F, t559: F, t2001: F, t305: F, t498: F, t552: F, t1596: F, t1594: F, t2410: F, t7228: F, t1969: F) -> (F, F, F, F, F, F, F) {
    let t39122 = t2411 * t7920 * t678;
    let t39141 = t1986 * t326 * t559 * t495;
    let t39171 = t2001 * t305 * t552 * t498;
    let t39183 = t1986 * t1596;
    let t39199 = t1986 * t1594;
    let t39207 = t2410 * t7228;
    let t39208 = t39207 * t1969;
    (t39122, t39141, t39171, t39183, t39199, t39207, t39208)
}
