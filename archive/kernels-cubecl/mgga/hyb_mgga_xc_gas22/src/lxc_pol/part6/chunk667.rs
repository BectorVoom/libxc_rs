//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 667/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk667<F: Float>(t222: F, t3309: F, t37: F, t2165: F, t2167: F, t3300: F, t251: F, t1333: F, t787: F) -> (F, F, F, F) {
    let t3311 = t222 * t37 * t3309;
    let t3313 = t2165 - F::cast_from(0.17808333333333333333e-1_f64) * t2167 - F::cast_from(0.17808333333333333333e-1_f64) * t3300 + F::cast_from(0.53425e-1_f64) * t3311;
    let t3315 = F::cast_from(0.621814e-1_f64) * t3313 * t251;
    let t3316 = t1333 * t787;
    (t3311, t3313, t3315, t3316)
}
