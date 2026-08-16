//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 472/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk472<F: Float>(t2194: F, t2195: F, t2164: F, t2167: F, t2178: F) -> (F, F, F) {
    let t2196 = t2194 * t2195;
    let t2198 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t2164;
    let t2200 = t2198 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t2167 + t2178;
    (t2196, t2198, t2200)
}
