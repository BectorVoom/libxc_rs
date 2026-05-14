//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 452/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk452<F: Float>(t143: F, t2065: F, t39: F, t2039: F, t2063: F, t2033: F, t698: F, t2013: F, t701: F, t2058: F, t2059: F, t571: F) -> (F, F, F, F, F, F, F) {
    let t145 = 0.135e1 < t143;
    let t2066 = t39 * t2065;
    let t2068 = t2063 * t2066 * t2039;
    let t2071 = t39 * t2033;
    let t2073 = t698 * t2071 * t2039;
    let t2077 = t698 * t701 * t2013;
    let t2080 = t2058 + t2059 / 81.0 - t571 * t2068 / 81.0 + t571 * t2073 / 27.0 - t571 * t2077 / 54.0;
    let t2081 = piecewise3(t145, t2080, 0.0);
    (t2066, t2068, t2071, t2073, t2077, t2080, t2081)
}
