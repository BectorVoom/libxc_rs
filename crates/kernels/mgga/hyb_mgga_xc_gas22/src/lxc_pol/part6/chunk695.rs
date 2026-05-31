//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 695/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk695<F: Float>(t3517: F, t967: F, t2521: F, t2457: F, t2527: F, t3461: F, t3472: F, t1414: F, t978: F) -> (F, F, F, F) {
    let t3518 = t3517 * t967;
    let t3520 = F::cast_from(0.16081979498692535067e2_f64) * t2521 * t3518;
    let t3524 = t2527 - F::cast_from(0.17123333333333333333e-1_f64) * t2457 - F::cast_from(0.17123333333333333333e-1_f64) * t3461 + F::cast_from(0.5137e-1_f64) * t3472;
    let t3527 = t1414 * t978;
    (t3518, t3520, t3524, t3527)
}
