//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 766/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk766<F: Float>(t291: F, t653: F, t2418: F, t297: F, t10122: F, t2165: F, t3247: F, t2300: F, t3188: F, t3187: F, t2885: F, t820: F, t3192: F, t3206: F, t763: F, t462: F, t762: F) -> (F, F, F, F, F, F, F) {
    let t10123 = t653 * t291;
    let t10125 = t10123 * t297 * t2418;
    let t10126 = t10122 * t10125;
    let t10128 = t2165 * t3247;
    let t10130 = t3188 * t2300;
    let t10131 = t3187 * t10130;
    let t10133 = t2885 * t820;
    let t10134 = t3192 * t10133;
    let t10137 = t763 * t3206;
    let t10139 = t462 * t762;
    (t10123, t10126, t10128, t10131, t10134, t10137, t10139)
}
