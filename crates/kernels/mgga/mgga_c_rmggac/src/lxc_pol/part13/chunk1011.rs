//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1011/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1011<F: Float>(t34884: F, t9123: F, t1240: F, t1971: F, t511: F, t558: F, t7230: F, t4601: F, t9008: F, t27036: F, t681: F, t26346: F, t7710: F) -> (F, F, F, F, F) {
    let t42144 = t34884 * t9123;
    let t42149 = t7230 * t1971 * t511 * t558 * t1240;
    let t42151 = t4601 * t9008;
    let t42156 = t27036 * t681;
    let t42159 = t26346 * t7710;
    (t42144, t42149, t42151, t42156, t42159)
}
