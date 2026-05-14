//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 691/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk691<F: Float>(t204: F, t205: F, t3026: F, t2173: F, t2175: F, t3017: F, t352: F, t1171: F, t832: F) -> (F, F, F, F) {
    let t3028 = t204 * t205 * t3026;
    let t3030 = t2173 - 0.17808333333333333333e-1 * t2175 - 0.17808333333333333333e-1 * t3017 + 0.53425e-1 * t3028;
    let t3032 = 0.621814e-1 * t3030 * t352;
    let t3033 = t1171 * t832;
    (t3028, t3030, t3032, t3033)
}
