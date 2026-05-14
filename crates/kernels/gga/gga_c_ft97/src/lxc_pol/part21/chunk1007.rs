//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1007/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1007<F: Float>(t15564: F, t15565: F, t2247: F, t172: F, t15576: F, t15569: F, t1526: F, t3009: F, t7705: F, t39430: F, t81: F, t15589: F, t342: F, t630: F, t2252: F, t4410: F) -> (F, F, F, F, F, F, F, F) {
    let t61123 = t15564 * t15565 * t2247;
    let t61128 = t15564 * t15565 * t172;
    let t61130 = t61128 * t15576 / 9.0;
    let t61132 = 2.0 / 27.0 * t61128 * t15569;
    let t61147 = t1526 * t7705 * t3009 / 18.0;
    let t61163 = t39430 * t81;
    let t61174 = t342 * t630 * t15589 / 6.0;
    let t61180 = t342 * t2252 * t4410;
    (t61123, t61128, t61130, t61132, t61147, t61163, t61174, t61180)
}
