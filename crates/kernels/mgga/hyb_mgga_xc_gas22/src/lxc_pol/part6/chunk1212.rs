//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1212/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1212<F: Float>(t24143: F, t3: F, t4002: F, t6012: F, t10288: F, t10293: F, t2002: F, t2028: F, t20292: F, t24140: F, t24142: F, t24149: F, t24154: F, t24158: F, t24161: F, t24163: F, t24186: F, t24205: F, t27275: F, t3171: F, t3925: F, t572: F, t8296: F) -> (F,) {
    let t28258 = t24143 * t3;
    let t28268 = t6012 * t4002;
    let t28274 = -2.0 / 81.0 * t24140 - 5.0 / 243.0 * t572 * t8296 * t10288 * t2002 - 40.0 / 729.0 * t572 * t24205 * t20292 * t3925 * t2028 + 2.0 / 27.0 * t572 * t3171 * t10293 * t2002 + 40.0 / 243.0 * t27275 * t24154 * t28258 - 16.0 / 27.0 * t27275 * t24149 * t28258 + 8.0 / 9.0 * t27275 * t24142 * t28258 - 4.0 / 729.0 * t28268 - 8.0 / 243.0 * t24158 - 2.0 / 81.0 * t24161 + 8.0 / 81.0 * t24163 + 28.0 / 729.0 * t24186;
    (t28274,)
}
