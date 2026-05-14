//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 433/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk433<F: Float>(t43: F, t1916: F, t1917: F, t1960: F, t615: F, t634: F, t72: F, t88: F, t29: F, t125: F, t26: F, t639: F, t667: F, t10: F, t18: F, t1877: F) -> (F, F, F, F, F, F) {
    let t44 = 0.135e1 <= t43;
    let t1964 = piecewise3(t44, t1916, -8.0 / 3.0 * t1917 * t88 - 16.0 / 3.0 * t615 * t634 - 8.0 / 3.0 * t72 * t1960);
    let t1965 = t29 * t1964;
    let t1966 = t1965 * t125;
    let t1967 = t26 * t1966;
    let t1970 = t639 * t667;
    let t1971 = t26 * t1970;
    let t1975 = t1877 * t10 * t18;
    (t1964, t1966, t1967, t1970, t1971, t1975)
}
